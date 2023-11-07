//! # lib-linux
//!
//! A wrapper library for linux functions

#![deny(missing_docs)]
#![feature(c_size_t)]

mod descriptor;
mod error;
mod networking;

pub mod raw;

pub use descriptor::*;
pub use error::*;
pub use networking::*;
