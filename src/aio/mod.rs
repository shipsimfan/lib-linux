//! Asynchronous input and output

mod functions;
mod structures;

pub use functions::{aio_error, aio_read, aio_return};
pub use structures::aiocb;
