//! # Raw Linux bindings
//!
//! The modules are organized like the header files and folders on Linux

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

// Folders
pub mod linux;
pub mod netinet;
pub mod sys;

// Header files
pub mod aio;
pub mod dlfcn;
pub mod errno;
pub mod fcntl;
pub mod locale;
pub mod netdb;
pub mod poll;
pub mod pthread;
pub mod signal;
pub mod stdlib;
pub mod string;
pub mod time;
pub mod unistd;

// Utilities for easier working with the raw bindings
mod utility;
pub use utility::*;
