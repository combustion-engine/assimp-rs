#[macro_use]
mod macros;

pub mod default;
pub mod stream;
pub mod read_only;
pub mod multi;

pub trait AssimpIO<'a> {
    fn get(&'a mut self) -> &'a mut ::ffi::AiFileIO;
}

pub use self::default::DefaultIO;
pub use self::stream::StreamIO;
pub use self::read_only::ReadOnlyStreamIO;
pub use self::multi::{MultiStreamIO, to_stream, to_write_stream};