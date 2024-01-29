//! Asynchronous input and output

mod functions;
mod structures;

pub use functions::{aio_error, aio_read};
pub use structures::aiocb;
