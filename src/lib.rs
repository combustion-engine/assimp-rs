#![feature(conservative_impl_trait)]
#![allow(dead_code)]

extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;
extern crate vec_map;
extern crate lazy;

#[cfg(feature = "compat")]
extern crate nalgebra;

#[cfg(feature = "compat")]
pub mod compat;

pub mod ffi;
#[macro_use]
pub mod error;
pub mod postprocess;
#[macro_use]
pub mod components;
pub mod scene;
pub mod formats;

pub use ffi::{AiVector3D, AiVector2D, AiMatrix3x3, AiMatrix4x4};
pub use error::*;
pub use postprocess::PostprocessEffect;
pub use components::*;
pub use scene::Scene;