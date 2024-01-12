//! # lib-linux
//!
//! A wrapper library for linux functions

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod descriptor;
mod error;
mod locale;
mod networking;

pub use raw;

pub use descriptor::*;
pub use error::*;
pub use locale::*;
pub use networking::*;
