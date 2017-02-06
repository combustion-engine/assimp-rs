use std::ptr;

use ::ffi::AiFileIO;

pub struct DefaultIO {
    io: AiFileIO,
}

impl DefaultIO {
    pub fn new() -> DefaultIO {
        DefaultIO {
            io: AiFileIO {
                open: procs::default_open_proc,
                close: procs::default_close_proc,
                user_data: ptr::null_mut(),
            }
        }
    }
}

impl Default for DefaultIO {
    fn default() -> DefaultIO {
        DefaultIO::new()
    }
}

impl<'a> super::AssimpIO<'a> for DefaultIO {
    fn get(&'a mut self) -> &'a mut AiFileIO {
        &mut self.io
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

    use ::ffi::{self, AiFile, AiFileIO, AiUserData};

    pub extern "C" fn default_open_proc(file_io: *mut AiFileIO, path: *const c_char, _mode: *const c_char) -> *mut AiFile {
        c_assert!(!file_io.is_null());
        c_assert!(!path.is_null());

        let path = if let Ok(path) = unsafe { CStr::from_ptr(path) }.to_str() { path } else {
            return ptr::null_mut();
        };

        let file = if let Ok(file) = File::open(path) { box file } else {
            return ptr::null_mut();
        };

        let ai_file = box AiFile {
            user_data: Box::into_raw(file) as AiUserData,
            read: default_read_proc,
            write: default_write_proc,
            tell: default_tell_proc,
            size: default_tell_size_proc,
            seek: default_seek_proc,
            flush: default_flush_proc,
        };

        Box::into_raw(ai_file)
    }

    pub extern "C" fn default_close_proc(file_io: *mut AiFileIO, file: *mut AiFile) {
        c_assert!(!file_io.is_null());
        c_assert!(!file.is_null());

        let ai_file = unsafe { Box::from_raw(file) };

        c_assert!(!ai_file.user_data.is_null());

        // Turn the file back into a box to drop it
        unsafe { Box::from_raw(ai_file.user_data as *mut File); }
    }

    pub extern "C" fn default_read_proc(file: *mut AiFile, buffer: *mut c_char, size: size_t, count: size_t) -> size_t {
        let mut file: &mut File = user_data!(file);

        let mut buffer = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize * count as usize) };

        match file.read(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to read data from file: {}", err);
            }
        }
    }

    pub extern "C" fn default_write_proc(file: *mut AiFile, buffer: *const c_char, size: size_t, count: size_t) -> size_t {
        let mut file: &mut File = user_data!(file);

        let buffer = unsafe { slice::from_raw_parts(buffer as *const u8, size as usize * count as usize) };

        match file.write(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to write data to file: {}", err);
            }
        }
    }

    pub extern "C" fn default_tell_proc(file: *mut AiFile) -> size_t {
        let mut file: &mut File = user_data!(file);

        match file.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos as size_t,
            Err(err) => {
                c_abort!("Failed to get current position: {}", err);
            }
        }
    }

    pub extern "C" fn default_tell_size_proc(file: *mut AiFile) -> size_t {
        let file: &mut File = user_data!(file);

        match file.metadata() {
            Ok(meta) => meta.len() as size_t,
            Err(err) => {
                c_abort!("Failed to get file metadata: {}", err);
            }
        }
    }

    pub extern "C" fn default_seek_proc(file: *mut AiFile, pos: size_t, origin: c_int) -> c_int {
        let mut file: &mut File = user_data!(file);

        let origin = match origin {
            ffi::AI_ORIGIN_SET => SeekFrom::Start(pos as u64),
            ffi::AI_ORIGIN_CUR => SeekFrom::Current(pos as i64),
            ffi::AI_ORIGIN_END => SeekFrom::End(pos as i64),
            _ => c_abort!("Invalid Seek origin"),
        };

        if file.seek(origin).is_ok() { ffi::AI_SUCCESS } else { ffi::AI_FAILURE }
    }

    pub extern "C" fn default_flush_proc(file: *mut AiFile) {
        let mut file: &mut File = user_data!(file);

        c_assert!(file.flush().is_ok())
    }
}