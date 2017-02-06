use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use ::ffi::{AiFileIO, AiUserData};

pub struct ReadOnlyStreamIO {
    io: AiFileIO,
}

impl Drop for ReadOnlyStreamIO {
    fn drop(&mut self) {
        assert!(!self.io.user_data.is_null());

        unsafe { Box::from_raw(self.io.user_data as *mut ReadOnlyStreamData); }
    }
}

impl<'a> super::AssimpIO<'a> for ReadOnlyStreamIO {
    fn get(&'a mut self) -> &'a mut AiFileIO {
        &mut self.io
    }
}

pub trait AiReadOnlyStream: Read + Seek + 'static {}

impl<T> AiReadOnlyStream for T where T: Read + Seek + 'static {}

struct ReadOnlyStreamData {
    stream: Option<Box<AiReadOnlyStream>>,
    stream_present: AtomicBool,
    hint: Option<PathBuf>,
}

impl ReadOnlyStreamIO {
    pub fn new<S: AiReadOnlyStream, P: AsRef<Path>>(stream: S, hint: Option<P>) -> ReadOnlyStreamIO {
        let stream_data = box ReadOnlyStreamData {
            stream: Some(box stream),
            stream_present: AtomicBool::new(true),
            hint: hint.map(|hint| hint.as_ref().into()),
        };

        ReadOnlyStreamIO {
            io: AiFileIO {
                open: self::procs::ro_stream_open_proc,
                close: self::procs::ro_stream_close_proc,
                user_data: Box::into_raw(stream_data) as AiUserData,
            }
        }
    }

    pub fn new_nohint<S: AiReadOnlyStream>(stream: S) -> ReadOnlyStreamIO {
        ReadOnlyStreamIO::new::<S, &'static str>(stream, None)
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

    use super::{ReadOnlyStreamData, AiReadOnlyStream};

    pub extern "C" fn ro_stream_open_proc(file_io: *mut AiFileIO, path: *const c_char, _mode: *const c_char) -> *mut AiFile {
        c_assert!(!file_io.is_null());

        let mut stream_data: &mut ReadOnlyStreamData = user_data!(file_io);

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

        let stream: Box<AiReadOnlyStream> = if let Some(stream) = stream {
            stream
        } else if let Ok(file) = File::open(path) {
            Box::new(file) as Box<AiReadOnlyStream>
        } else {
            return ptr::null_mut();
        };

        let ai_file = box AiFile {
            user_data: Box::into_raw(box stream) as AiUserData,
            read: ro_stream_read_proc,
            write: ro_stream_write_proc,
            tell: ro_stream_tell_proc,
            size: ro_stream_tell_size_proc,
            seek: ro_stream_seek_proc,
            flush: ro_stream_flush_proc,
        };

        Box::into_raw(ai_file)
    }

    pub extern "C" fn ro_stream_close_proc(file_io: *mut AiFileIO, file: *mut AiFile) {
        c_assert!(!file_io.is_null());
        c_assert!(!file.is_null());

        let ai_file = unsafe { Box::from_raw(file) };

        c_assert!(!ai_file.user_data.is_null());

        // Turn the file back into a box to drop it
        unsafe { Box::from_raw(ai_file.user_data as *mut Box<AiReadOnlyStream>) };
    }

    pub extern "C" fn ro_stream_read_proc(file: *mut AiFile, buffer: *mut c_char, size: size_t, count: size_t) -> size_t {
        let mut stream: &mut Box<AiReadOnlyStream> = user_data!(file);

        let mut buffer = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize * count as usize) };

        match stream.read(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to read data from file: {}", err);
            }
        }
    }

    pub extern "C" fn ro_stream_write_proc(_: *mut AiFile, _: *const c_char, _: size_t, _: size_t) -> size_t {
        0
    }

    pub extern "C" fn ro_stream_tell_proc(file: *mut AiFile) -> size_t {
        let stream: &mut Box<AiReadOnlyStream> = user_data!(file);

        match stream.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos as size_t,
            Err(err) => c_abort!("Failed to set stream position: {}", err),
        }
    }

    pub extern "C" fn ro_stream_tell_size_proc(file: *mut AiFile) -> size_t {
        let mut stream: &mut Box<AiReadOnlyStream> = user_data!(file);

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

    pub extern "C" fn ro_stream_seek_proc(file: *mut AiFile, pos: size_t, origin: c_int) -> c_int {
        let mut stream: &mut Box<AiReadOnlyStream> = user_data!(file);

        let origin = match origin {
            ffi::AI_ORIGIN_SET => SeekFrom::Start(pos as u64),
            ffi::AI_ORIGIN_CUR => SeekFrom::Current(pos as i64),
            ffi::AI_ORIGIN_END => SeekFrom::End(pos as i64),
            _ => c_abort!("Invalid Seek origin"),
        };

        if stream.seek(origin).is_ok() { ffi::AI_SUCCESS } else { ffi::AI_FAILURE }
    }

    pub extern "C" fn ro_stream_flush_proc(_: *mut AiFile) {}
}