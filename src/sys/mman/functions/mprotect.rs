use crate::c_size_t;
use core::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EACCES, EINVAL, ENOMEM},
    sys::mman::{PROT_EXEC, PROT_NONE, PROT_READ, PROT_WRITE},
};

extern "C" {
    /// [`mprotect`] changes the access protections for the calling process's memory pages
    /// containing any part of the address range in the interval [`addr`, `addr + len - 1`]. `addr`
    /// must be aligned to a page boundary.
    ///
    /// If the calling process tries to access memory in a manner that violates the protections,
    /// then the kernel generates a [`SIGSEGV`] signal for the process.
    ///
    /// `prot` is a combination of the following access flags:
    ///  * [`PROT_NONE`] - The memory cannot be accessed at all.
    ///  * [`PROT_READ`] - The memory can be read.
    ///  * [`PROT_WRITE`] - The memory can be modified.
    ///  * [`PROT_EXEC`] - The memory can be executed.
    ///
    /// # Return Value
    /// On success, [`mprotect`] returns zero. On error, this system call returns -1, and [`errno`]
    /// is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EACCES`] - The memory cannot be given the specified access. This can happen, for
    ///                 example, if you [`mmap`] a file to which you have read-only access, then
    ///                 ask [`mprotect`] to mark it [`PROT_WRITE`].
    ///  * [`EINVAL`] - `addr` is not a valid pointer, or not a multiple of the system page size.
    ///  * [`EINVAL`] - Both [`PROT_GROWSUP`] and [`PROT_GROWSDOWN`] were specified in `prot`.
    ///  * [`EINVAL`] - Invalid flags specified in `prot`.
    ///  * [`ENOMEM`] - Internal kernel structures could not be allocated.
    ///  * [`ENOMEM`] - Addresses in the range [`addr`,`addr + len - 1`] are invalid for the
    ///                 address space of the process, or specify one or more pages that are not
    ///                 mapped.
    ///  * [`ENOMEM`] - Changing the protection of a memory region would result in the total number
    ///                 of mappings with distinct attributes (e.g., read versus read/write
    ///                 protection) exceeding the allowed maximum. (For example, making the
    ///                 protection of a range [`PROT_READ`] in the middle of a region currently
    ///                 protected as [`PROT_READ`] | [`PROT_WRITE`] would result in three mappings:
    ///                 two read/write mappings at each end and a read-only mapping in the middle.)
    pub fn mprotect(addr: *mut c_void, len: c_size_t, prot: c_int) -> c_int;
}
