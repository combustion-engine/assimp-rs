use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use ::ffi::{AiFileIO, AiUserData};

use super::read_only::AiReadOnlyStream;

pub struct StreamIO {
    io: AiFileIO,
}

impl Drop for StreamIO {
    fn drop(&mut self) {
        assert!(!self.io.user_data.is_null());

        unsafe { Box::from_raw(self.io.user_data as *mut StreamData); }
    }
}

impl<'a> super::AssimpIO<'a> for StreamIO {
    fn get(&'a mut self) -> &'a mut AiFileIO {
        &mut self.io
    }
}

pub trait AiStream: AiReadOnlyStream + Write {}

impl<T> AiStream for T where T: AiReadOnlyStream + Write {}

struct StreamData {
    stream: Option<Box<AiStream>>,
    stream_present: AtomicBool,
    hint: Option<PathBuf>,
}

impl StreamIO {
    pub fn new<S: AiStream, P: AsRef<Path>>(stream: S, hint: Option<P>) -> StreamIO {
        let stream_data = box StreamData {
            stream: Some(box stream),
            stream_present: AtomicBool::new(true),
            hint: hint.map(|hint| hint.as_ref().into()),
        };

        StreamIO {
            io: AiFileIO {
                open: self::procs::stream_open_proc,
                close: self::procs::stream_close_proc,
                user_data: Box::into_raw(stream_data) as AiUserData,
            }
        }
    }

    pub fn new_nohint<S: AiStream>(stream: S) -> StreamIO {
        StreamIO::new::<S, &'static str>(stream, None)
    }
}

pub mod procs {
    use libc::{c_char, size_t, c_int};

    use std::io::prelude::*;
    use std::io::SeekFrom;
    use std::fs::File;
    use std::ffi::CStr;
    use std::ptr;
    use std::slice;
    use std::sync::atomic::Ordering;

    use ::ffi::{self, AiFile, AiFileIO, AiUserData};

    use super::{StreamData, AiStream};

    pub extern "C" fn stream_open_proc(file_io: *mut AiFileIO, path: *const c_char, _mode: *const c_char) -> *mut AiFile {
        c_assert!(!file_io.is_null());

        let mut stream_data: &mut StreamData = user_data!(file_io);

        let path = if let Ok(path) = unsafe { CStr::from_ptr(path) }.to_str() { path } else {
            return ptr::null_mut();
        };

        let mut stream = None;

        if let Some(ref hint) = stream_data.hint {
            if path == hint.as_os_str() {
                if stream_data.stream_present.swap(false, Ordering::SeqCst) {
                    stream = stream_data.stream.take()
                }
            }
        }

        let stream = if let Some(stream) = stream {
            stream
        } else if let Ok(file) = File::open(path) {
            box file as Box<AiStream>
        } else {
            return ptr::null_mut()
        };

        let ai_file = box AiFile {
            user_data: Box::into_raw(box stream) as AiUserData,
            write: stream_write_proc,
            flush: stream_flush_proc,
            read: stream_read_proc,
            tell: stream_tell_proc,
            size: stream_tell_size_proc,
            seek: stream_seek_proc,
        };

        Box::into_raw(ai_file)
    }

    pub extern "C" fn stream_close_proc(file_io: *mut AiFileIO, file: *mut AiFile) {
        c_assert!(!file_io.is_null());
        c_assert!(!file.is_null());

        let ai_file = unsafe { Box::from_raw(file) };

        c_assert!(!ai_file.user_data.is_null());

        // Turn the file back into a box to drop it
        unsafe { Box::from_raw(ai_file.user_data as *mut Box<AiStream>) };
    }

    pub extern "C" fn stream_read_proc(file: *mut AiFile, buffer: *mut c_char, size: size_t, count: size_t) -> size_t {
        let mut stream: &mut Box<AiStream> = user_data!(file);

        let mut buffer = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize * count as usize) };

        match stream.read(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to read data from file: {}", err);
            }
        }
    }

    pub extern "C" fn stream_write_proc(file: *mut AiFile, buffer: *const c_char, size: size_t, count: size_t) -> size_t {
        let mut stream: &mut Box<AiStream> = user_data!(file);

        let buffer = unsafe { slice::from_raw_parts(buffer as *const u8, size as usize * count as usize) };

        match stream.write(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to write data to file: {}", err);
            }
        }
    }

    pub extern "C" fn stream_tell_proc(file: *mut AiFile) -> size_t {
        let stream: &mut Box<AiStream> = user_data!(file);

        match stream.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos as size_t,
            Err(err) => c_abort!("Failed to set stream position: {}", err),
        }
    }

    pub extern "C" fn stream_tell_size_proc(file: *mut AiFile) -> size_t {
        let mut stream: &mut Box<AiStream> = user_data!(file);

        let cur = match stream.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos,
            Err(err) => c_abort!("Failed to get stream size: {}", err),
        };

        let size = match stream.seek(SeekFrom::End(0)) {
            Ok(pos) => pos,
            Err(err) => c_abort!("Failed to get stream size: {}", err),
        };

        match stream.seek(SeekFrom::Start(cur)) {
            Ok(_) => size as size_t,
            Err(err) => c_abort!("Failed to get stream size: {}", err),
        }
    }

    pub extern "C" fn stream_seek_proc(file: *mut AiFile, pos: size_t, origin: c_int) -> c_int {
        let mut stream: &mut Box<AiStream> = user_data!(file);

        let origin = match origin {
            ffi::AI_ORIGIN_SET => SeekFrom::Start(pos as u64),
            ffi::AI_ORIGIN_CUR => SeekFrom::Current(pos as i64),
            ffi::AI_ORIGIN_END => SeekFrom::End(pos as i64),
            _ => c_abort!("Invalid Seek origin"),
        };

        if stream.seek(origin).is_ok() { ffi::AI_SUCCESS } else { ffi::AI_FAILURE }
    }

    pub extern "C" fn stream_flush_proc(file: *mut AiFile) {
        let mut stream: &mut Box<AiStream> = user_data!(file);

        c_assert!(stream.flush().is_ok())
    }
}