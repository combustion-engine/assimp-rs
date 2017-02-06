use std::io;
use std::path::Path;

use ::ffi::{AiFileIO, AiUserData};

pub struct MultiStreamIO {
    io: AiFileIO,
}

impl Drop for MultiStreamIO {
    fn drop(&mut self) {
        assert!(!self.io.user_data.is_null());

        unsafe { Box::from_raw(self.io.user_data as *mut MultiStreamData); }
    }
}

impl<'a> super::AssimpIO<'a> for MultiStreamIO {
    fn get(&'a mut self) -> &'a mut AiFileIO {
        &mut self.io
    }
}

use super::read_only::AiReadOnlyStream;
use super::stream::AiStream;

struct MultiStreamData {
    open_callback: Box<FnMut(&Path) -> io::Result<procs::StreamMode>>
}

pub fn to_stream<S: AiReadOnlyStream>(stream: S) -> Box<AiReadOnlyStream> {
    box stream as Box<AiReadOnlyStream>
}

pub fn to_write_stream<S: AiStream>(stream: S) -> Box<AiStream> {
    box stream as Box<AiStream>
}

impl MultiStreamIO {
    pub fn read<F>(mut f: F) -> MultiStreamIO where F: FnMut(&Path) -> io::Result<Box<AiReadOnlyStream>> + 'static {
        let stream_data = box MultiStreamData {
            open_callback: box move |path| {
                f(path).map(procs::StreamMode::Read)
            },
        };

        MultiStreamIO {
            io: AiFileIO {
                open: self::procs::multi_stream_open_proc,
                close: ::io::read_only::procs::ro_stream_close_proc,
                user_data: Box::into_raw(stream_data) as AiUserData,
            }
        }
    }

    pub fn write<F>(mut f: F) -> MultiStreamIO where F: FnMut(&Path) -> io::Result<Box<AiStream>> + 'static {
        let stream_data = box MultiStreamData {
            open_callback: box move |path| {
                f(path).map(procs::StreamMode::Write)
            },
        };

        MultiStreamIO {
            io: AiFileIO {
                open: self::procs::multi_stream_open_proc,
                close: ::io::stream::procs::stream_close_proc,
                user_data: Box::into_raw(stream_data) as AiUserData,
            }
        }
    }
}

pub mod procs {
    use libc::c_char;

    use std::io::prelude::*;
    use std::path::Path;
    use std::ffi::CStr;
    use std::ptr;

    use ::ffi::{AiFile, AiFileIO, AiUserData};

    use ::io::stream::AiStream;
    use ::io::read_only::AiReadOnlyStream;

    use super::MultiStreamData;

    pub enum StreamMode {
        Read(Box<AiReadOnlyStream>),
        Write(Box<AiStream>),
    }

    pub extern "C" fn multi_stream_open_proc(file_io: *mut AiFileIO, path: *const c_char, _mode: *const c_char) -> *mut AiFile {
        let mut stream_data: &mut MultiStreamData = user_data!(file_io);

        let path = if let Ok(path) = unsafe { CStr::from_ptr(path).to_str() } { Path::new(path) } else {
            return ptr::null_mut();
        };

        if let Ok(stream_mode) = (stream_data.open_callback)(path) {
            let ai_file = match stream_mode {
                StreamMode::Read(read_stream) => {
                    box AiFile {
                        user_data: Box::into_raw(box read_stream) as AiUserData,
                        read: ::io::read_only::procs::ro_stream_read_proc,
                        write: ::io::read_only::procs::ro_stream_write_proc,
                        tell: ::io::read_only::procs::ro_stream_tell_proc,
                        size: ::io::read_only::procs::ro_stream_tell_size_proc,
                        seek: ::io::read_only::procs::ro_stream_seek_proc,
                        flush: ::io::read_only::procs::ro_stream_flush_proc,
                    }
                },
                StreamMode::Write(write_stream) => {
                    box AiFile {
                        user_data: Box::into_raw(box write_stream) as AiUserData,
                        read: ::io::stream::procs::stream_read_proc,
                        write: ::io::stream::procs::stream_write_proc,
                        tell: ::io::stream::procs::stream_tell_proc,
                        size: ::io::stream::procs::stream_tell_size_proc,
                        seek: ::io::stream::procs::stream_seek_proc,
                        flush: ::io::stream::procs::stream_flush_proc,
                    }
                }
            };

            Box::into_raw(ai_file)
        } else {
            ptr::null_mut()
        }
    }
}