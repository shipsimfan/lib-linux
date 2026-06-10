use crate::c_size_t;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{errno::errno, signal::SIGSEGV, sys::mman::mmap};

unsafe extern "C" {
    /// Unmap pages of memory
    ///
    /// # Description
    /// The [`munmap`] function shall remove any mappings for those entire pages containing any
    /// part of the address space of the process starting at `addr` and continuing for `len` bytes.
    /// Further references to these pages shall result in the generation of a [`SIGSEGV`] signal to
    /// the process. If there are no mappings in the specified address range, then [`munmap`] has
    /// no effect.
    ///
    /// The implementation shall require that `addr` be a multiple of the page size ([`PAGESIZE`]).
    ///
    /// If a mapping to be removed was private, any modifications made in this address range shall
    /// be discarded.
    ///
    /// Any memory locks (see [`mlock`] and [`mlockall`]) associated with this address range shall
    /// be removed, as if by an appropriate call to [`munlock`].
    ///
    /// If a mapping removed from a typed memory object causes the corresponding address range of
    /// the memory pool to be inaccessible by any process in the system except through allocatable
    /// mappings (that is, mappings of typed memory objects opened with the
    /// [`POSIX_TYPED_MEM_MAP_ALLOCATABLE`] flag), then that range of the memory pool shall become
    /// deallocated and may become available to satisfy future typed memory allocation requests.
    ///
    /// A mapping removed from a typed memory object opened with the
    /// [`POSIX_TYPED_MEM_MAP_ALLOCATABLE`] flag shall not affect in any way the availability of
    /// that typed memory for allocation.
    ///
    /// The behavior of this function is unspecified if the mapping was not established by a call
    /// to [`mmap`].
    ///
    /// # Return Value
    /// Upon successful completion, [`munmap`] shall return 0; otherwise, it shall return -1 and
    /// set [`errno`] to indicate the error.
    pub fn munmap(addr: *mut c_void, length: c_size_t) -> c_int;
}
