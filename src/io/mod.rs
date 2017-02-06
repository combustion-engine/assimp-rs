#[macro_use]
mod macros;

pub mod default;
pub mod stream;

pub trait AssimpIO<'a> {
    fn get(&'a mut self) -> &'a mut ::ffi::AiFileIO;
}

pub use self::default::DefaultIO;
pub use self::stream::StreamIO;