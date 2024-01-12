//! # Raw Linux bindings
//!
//! The modules are organized like the header files and folders on Linux

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_size_t)]

// Folders
pub mod linux;
pub mod netinet;
pub mod sys;

// Header files
pub mod errno;
pub mod locale;
pub mod string;
pub mod unistd;
