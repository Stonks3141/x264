//! A safe wrapper over the x264 video encoding library.

#![no_std]
#![warn(missing_docs)]

extern crate x264_sys;

use x264_sys::x264;

mod colorspace;
mod data;
mod encoder;
mod error;
mod image;
mod picture;
mod setup;

pub use colorspace::*;
pub use data::*;
pub use encoder::*;
pub use error::*;
pub use image::*;
pub use picture::*;
pub use setup::*;
