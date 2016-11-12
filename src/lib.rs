#![allow(dead_code, unused_imports)]

extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;

#[cfg(feature = "compat")]
extern crate nalgebra;

#[cfg(feature = "compat")]
pub mod compat;

pub mod ffi;
#[macro_use]
pub mod error;
pub mod postprocess;
pub mod components;
pub mod scene;

pub use ffi::{AiVector3D, AiVector2D, AiMatrix3x3, AiMatrix4x4};
pub use error::*;
pub use postprocess::PostprocessEffect;
pub use components::*;
pub use scene::Scene;