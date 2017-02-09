use std::path::Path;
use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::fs::File;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::ops::{Deref, DerefMut};

use ::ffi::{AiFileIO, AiUserData};

macro_rules! user_data {
    ($file:expr) => {unsafe {
        c_assert!(!$file.is_null());

        let user_data = (*$file).user_data as *mut _;

        c_assert!(!user_data.is_null());

        &mut *user_data
    }}
}

/// Represents a type which can provide access to a set of callbacks which can
/// be used within Assimp's custom IO API.
///
/// It is inadvisable to use this directly, as it requires knowledge of the C API.
/// Try using `CustomIO` instead.
pub trait AssimpIO<'a> {
    fn get(&'a mut self) -> &'a mut ::ffi::AiFileIO;
}

/// Some type that is `Seek + Read + Write` and `'static`
///
/// Automatically derived by any type that fulfils those conditions
pub trait IOStream: Seek + Read + Write + 'static {}

impl<T> IOStream for T where T: Seek + Read + Write + 'static {}

/// This is a `Write`-able wrapper for `Read`-only streams that just performs no-ops on writes
///
/// Although usually a bad idea, the underlying C-API for Assimp custom IO has no notion of read-only
/// streams, so we just need to ignore write requests.
///
/// **WARNING**: Please do **NOT** use this outside of custom Assimp IO.
/// This is only a direct result of the C-APIs nature.
pub struct ReadOnlyStream<R>(R) where R: Seek + Read + 'static;

impl<R> ReadOnlyStream<R> where R: Seek + Read + 'static {
    /// Create a new `ReadOnlyStream` from the given stream.
    pub fn new(stream: R) -> ReadOnlyStream<R> {
        ReadOnlyStream(stream)
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.0
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.0
    }

    /// Unwraps the `ReadOnlyStream` and returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> Deref for ReadOnlyStream<R> where R: Seek + Read + 'static {
    type Target = R;

    fn deref(&self) -> &R {
        &self.0
    }
}

impl<R> DerefMut for ReadOnlyStream<R> where R: Seek + Read + 'static {
    fn deref_mut(&mut self) -> &mut R {
        &mut self.0
    }
}

impl<R> Read for ReadOnlyStream<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<R> Seek for ReadOnlyStream<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl<R> Write for ReadOnlyStream<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<R> BufRead for ReadOnlyStream<R> where R: Seek + Read + BufRead + 'static {
    #[inline(always)]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.0.fill_buf()
    }

    #[inline(always)]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

impl<R> AsRef<R> for ReadOnlyStream<R> where R: Seek + Read + 'static {
    fn as_ref(&self) -> &R {
        self.get_ref()
    }
}

impl<R> AsMut<R> for ReadOnlyStream<R> where R: Seek + Read + 'static {
    fn as_mut(&mut self) -> &mut R {
        self.get_mut()
    }
}

/// Provides methods for opening and closing streams
pub trait IOHandler<S: IOStream>: Send + Sync + 'static {
    fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<S>;
    fn close(&self, stream: S) -> io::Result<()>;
}

/// Simple `File`-based `IOHandler`
pub struct DefaultIOHandler;

impl IOHandler<File> for DefaultIOHandler {
    fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        File::open(path)
    }

    fn close(&self, _: File) -> io::Result<()> {
        Ok(())
    }
}

/// Callback-based `IOHandler`, where the callback is invoked on `open`
///
/// `close` does nothing, and streams should be properly closed on `drop`
pub struct CallbackIOHandler<S: IOStream> {
    callback: Mutex<Box<FnMut(&Path) -> io::Result<S>>>,
}

impl<S: IOStream> CallbackIOHandler<S> {
    pub fn new<F>(cb: F) -> CallbackIOHandler<S> where F: FnMut(&Path) -> io::Result<S> + 'static {
        CallbackIOHandler { callback: Mutex::new(box cb) }
    }
}

impl<S: IOStream> IOHandler<S> for CallbackIOHandler<S> {
    fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<S> {
        if let Ok(mut cb) = self.callback.lock() {
            // Invoke the callback
            (&mut *cb)(path.as_ref())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "PoisonError"))
        }
    }

    fn close(&self, _: S) -> io::Result<()> {
        Ok(())
    }
}

unsafe impl<S: IOStream> Send for CallbackIOHandler<S> {}

unsafe impl<S: IOStream> Sync for CallbackIOHandler<S> {}

/// Represents some custom input/output system for use with Assimp
pub struct CustomIO<S, H> where S: IOStream, H: IOHandler<S> {
    io: AiFileIO,
    _types: PhantomData<(S, H)>,
}

impl<'a, S, H> AssimpIO<'a> for CustomIO<S, H> where S: IOStream, H: IOHandler<S> {
    /// Gets a reference to the `AiFileIO` handler
    fn get(&'a mut self) -> &'a mut AiFileIO {
        &mut self.io
    }
}

impl<S, H> Drop for CustomIO<S, H> where S: IOStream, H: IOHandler<S> {
    fn drop(&mut self) {
        assert!(!self.io.user_data.is_null());

        // Convert user_data back into H to drop it
        unsafe { Box::from_raw(self.io.user_data as *mut H); }
    }
}

impl<S, H> CustomIO<S, H> where S: IOStream, H: IOHandler<S> {
    /// Create a new `CustomIO` instance using the given `IOHandler`
    pub fn new(handler: H) -> CustomIO<S, H> {
        CustomIO {
            io: AiFileIO {
                open: procs::open_proc::<S, H>,
                close: procs::close_proc::<S, H>,
                user_data: Box::into_raw(box handler) as AiUserData,
            },
            _types: PhantomData,
        }
    }
}

impl Default for CustomIO<File, DefaultIOHandler> {
    /// Create a default `File`-based `CustomIO` instance
    fn default() -> CustomIO<File, DefaultIOHandler> {
        CustomIO::new(DefaultIOHandler)
    }
}

impl<S: IOStream> CustomIO<S, CallbackIOHandler<S>> {
    /// Create a new `CustomIO` with a `CallbackIOHandler` directly from a provided callback.
    pub fn callback<F>(cb: F) -> CustomIO<S, CallbackIOHandler<S>> where F: FnMut(&Path) -> io::Result<S> + 'static {
        CustomIO::new(CallbackIOHandler::new(cb))
    }
}

mod procs {
    use libc::{c_char, size_t, c_int};

    use std::io::prelude::*;
    use std::io::SeekFrom;
    use std::path::Path;
    use std::ffi::CStr;
    use std::ptr;
    use std::slice;

    use ::ffi::{self, AiFile, AiFileIO, AiUserData};

    use super::{IOStream, IOHandler};

    /// Opens a new file via the given handler and boxes it up (and subsequently turns it into a raw pointer) to be used later
    ///
    /// By using template parameters on the procedures, we don't need to fool around with trait objects and double boxing
    pub extern "C" fn open_proc<S, H>(file_io: *mut AiFileIO, path: *const c_char, _mode: *const c_char) -> *mut AiFile where S: IOStream, H: IOHandler<S> {
        let handler: &mut H = user_data!(file_io);

        c_assert!(!path.is_null());

        let path = if let Ok(path) = unsafe { CStr::from_ptr(path).to_str() } { Path::new(path) } else {
            return ptr::null_mut();
        };

        let stream: S = if let Ok(stream) = handler.open(path) { stream } else {
            return ptr::null_mut();
        };

        Box::into_raw(box AiFile {
            user_data: Box::into_raw(box stream) as AiUserData,
            read: read_proc::<S>,
            write: write_proc::<S>,
            tell: tell_proc::<S>,
            size: tell_size_proc::<S>,
            seek: seek_proc::<S>,
            flush: flush_proc::<S>,
        })
    }

    /// Closes a stream by unboxing it and passing it to the handler's close function, which usually just drops it
    pub extern "C" fn close_proc<S, H>(file_io: *mut AiFileIO, file: *mut AiFile) where S: IOStream, H: IOHandler<S> {
        let handler: &mut H = user_data!(file_io);

        c_assert!(!file.is_null());

        let file = unsafe { Box::from_raw(file) };

        c_assert!(!file.user_data.is_null());

        let file = unsafe { Box::from_raw(file.user_data as *mut S) };

        c_assert!(handler.close(*file).is_ok());
    }

    /// Reads bytes from the stream into the given buffer
    pub extern "C" fn read_proc<S>(file: *mut AiFile, buffer: *mut c_char, size: size_t, count: size_t) -> size_t where S: IOStream {
        c_assert!(!buffer.is_null());

        let mut stream: &mut S = user_data!(file);

        let mut buffer = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize * count as usize) };

        match stream.read(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to read data from stream: {}", err);
            }
        }
    }

    /// Writes some bytes to the stream, returning the number of bytes written.
    pub extern "C" fn write_proc<S>(file: *mut AiFile, buffer: *const c_char, size: size_t, count: size_t) -> size_t where S: IOStream {
        c_assert!(!buffer.is_null());

        let mut stream: &mut S = user_data!(file);

        let buffer = unsafe { slice::from_raw_parts(buffer as *const u8, size as usize * count as usize) };

        match stream.write(buffer) {
            Ok(amt) => amt as size_t,
            Err(err) => {
                c_abort!("Failed to write data to stream: {}", err);
            }
        }
    }

    /// Gets the current position of the stream
    pub extern "C" fn tell_proc<S>(file: *mut AiFile) -> size_t where S: IOStream {
        let mut stream: &mut S = user_data!(file);

        match stream.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos as size_t,
            Err(err) => {
                c_abort!("Failed to get current stream position: {}", err);
            }
        }
    }

    /// Determine the overall length of the stream by seeking to the end and getting the position there.
    pub extern "C" fn tell_size_proc<S>(file: *mut AiFile) -> size_t where S: IOStream {
        let mut stream: &mut S = user_data!(file);

        // Store current position
        let cur = match stream.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos,
            Err(err) => c_abort!("Failed to get stream size: {}", err),
        };

        // Seek to the end to get total size
        let size = match stream.seek(SeekFrom::End(0)) {
            Ok(pos) => pos,
            Err(err) => c_abort!("Failed to get stream size: {}", err),
        };

        // Return the stream position back to the original
        match stream.seek(SeekFrom::Start(cur)) {
            Ok(_) => size as size_t,
            Err(err) => c_abort!("Failed to get stream size: {}", err),
        }
    }

    /// Seeks to a position in the stream. `S` must be `Seek`, so this is easy.
    pub extern "C" fn seek_proc<S>(file: *mut AiFile, pos: size_t, origin: c_int) -> c_int where S: IOStream {
        let mut stream: &mut S = user_data!(file);

        let origin = match origin {
            ffi::AI_ORIGIN_SET => SeekFrom::Start(pos as u64),
            ffi::AI_ORIGIN_CUR => SeekFrom::Current(pos as i64),
            ffi::AI_ORIGIN_END => SeekFrom::End(pos as i64),
            _ => c_abort!("Invalid Seek origin"),
        };

        // This procedure allows returning success and failure values
        if stream.seek(origin).is_ok() { ffi::AI_SUCCESS } else { ffi::AI_FAILURE }
    }

    /// Simply flushes the stream
    pub extern "C" fn flush_proc<S>(file: *mut AiFile) where S: IOStream {
        let mut stream: &mut S = user_data!(file);

        c_assert!(stream.flush().is_ok())
    }
}