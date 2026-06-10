use crate::{c_size_t, sys::types::off_t};
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{EBADF, EINVAL},
    signal::SIGBUS,
    sys::mman::{
        MAP_FAILED, MAP_FIXED, MAP_PRIVATE, MAP_SHARED, PROT_EXEC, PROT_NONE, PROT_READ,
        PROT_WRITE, munmap,
    },
    unistd::close,
};

unsafe extern "C" {
    /// Map pages of memory
    ///
    /// # Description
    /// The [`mmap`] function shall establish a mapping between a process' address space and a
    /// file, shared memory object, or typed memory object.
    ///
    /// The [`mmap`] function shall establish a mapping between the address space of the process at
    /// the address returned for `length` bytes to the memory object represented by the file
    /// descriptor `fd` at offset `offset` for `length` bytes. The value returned is an
    /// implementation-defined function of the parameter `addr` and the values of `flags`, further
    /// described below. The address range starting at the return value and continuing for `length`
    /// bytes shall be legitimate for the possible (not necessarily current) address space of the
    /// process. The range of bytes starting at `offset` and continuing for `length` bytes shall be
    /// legitimate for the possible (not necessarily current) offsets in the file, shared memory
    /// object, or typed memory object represented by `fd`.
    ///
    /// If `fd` represents a typed memory object opened with either the
    /// [`POSIX_TYPED_MEM_ALLOCATE`] flag or the [`POSIX_TYPED_MEM_ALLOCATE_CONTIG`] flag, the
    /// memory object to be mapped shall be that portion of the typed memory object allocated by
    /// the implementation as specified below. In this case, if `offset` is non-zero, the behavior
    /// of [`mmap`] is undefined. If `fd` refers to a valid typed memory object that is not
    /// accessible from the calling process, [`mmap`] shall fail.
    ///
    /// The mapping established by [`mmap`] shall replace any previous mappings for those whole
    /// pages containing any part of the address space of the process starting at the return value
    /// and continuing for `length` bytes.
    ///
    /// If the size of the mapped file changes after the call to [`mmap`] as a result of some other
    /// operation on the mapped file, the effect of references to portions of the mapped region
    /// that correspond to added or removed portions of the file is unspecified.
    ///
    /// The [`mmap`] function shall be supported for regular files, shared memory objects, and
    /// typed memory objects. Support for any other type of file is unspecified.
    ///
    /// The parameter `prot` determines whether read, write, execute, or some combination of
    /// accesses are permitted to the data being mapped. The `prot` shall be either [`PROT_NONE`]
    /// or the bitwise-inclusive OR of one or more of the other `flags` in the following table:
    ///  * [`PROT_WRITE`] - Data can be written.
    ///  * [`PROT_EXEC`] - Data can be executed.
    ///  * [`PROT_NONE`] - Data cannot be accessed.
    ///
    /// If an implementation cannot support the combination of access types specified by `prot`,
    /// the call to [`mmap`] shall fail.
    ///
    /// An implementation may permit accesses other than those specified by `prot`; however, if the
    /// Memory Protection option is supported, the implementation shall not permit a write to
    /// succeed where [`PROT_WRITE`] has not been set or shall not permit any access where
    /// [`PROT_NONE`] alone has been set. The implementation shall support at least the following
    /// values of `prot`: [`PROT_NONE`], [`PROT_READ`], [`PROT_WRITE`], and the bitwise-inclusive
    /// OR of [`PROT_READ`] and [`PROT_WRITE`]. If the Memory Protection option is not supported,
    /// the result of any access that conflicts with the specified protection is undefined. The
    /// file descriptor `fd` shall have been opened with read permission, regardless of the
    /// protection options specified. If [`PROT_WRITE`] is specified, the application shall ensure
    /// that it has opened the file descriptor `fd` with write permission unless [`MAP_PRIVATE`] is
    /// specified in the `flags` parameter as described below.
    ///
    /// The parameter `flags` provides other information about the handling of the mapped data. The
    /// value of `flags` is the bitwise-inclusive OR of these options:
    ///  * [`MAP_PRIVATE`] - Changes are private.
    ///  * [`MAP_FIXED`] - Interpret `addr` exactly.
    ///
    /// Implementations that do not support the Memory Mapped Files option are not required to
    /// support [`MAP_PRIVATE`].
    ///
    /// It is implementation-defined whether [`MAP_FIXED`] shall be supported. [`MAP_FIXED`] shall
    /// be supported on XSI-conformant systems.
    ///
    /// [`MAP_SHARED`] and [`MAP_PRIVATE`] describe the disposition of write references to the
    /// memory object. If [`MAP_SHARED`] is specified, write references shall change the underlying
    /// object. If [`MAP_PRIVATE`] is specified, modifications to the mapped data by the calling
    /// process shall be visible only to the calling process and shall not change the underlying
    /// object. It is unspecified whether modifications to the underlying object done after the
    /// [`MAP_PRIVATE`] mapping is established are visible through the [`MAP_PRIVATE`] mapping.
    /// Either [`MAP_SHARED`] or [`MAP_PRIVATE`] can be specified, but not both. The mapping type
    /// is retained across [`fork`].
    ///
    /// When `fd` represents a typed memory object opened with either the
    /// [`POSIX_TYPED_MEM_ALLOCATE`] flag or the [`POSIX_TYPED_MEM_ALLOCATE_CONTIG`] flag, [`mmap`]
    /// shall, if there are enough resources available, map `length` bytes allocated from the
    /// corresponding typed memory object which were not previously allocated to any process in any
    /// processor that may access that typed memory object. If there are not enough resources
    /// available, the function shall fail. If `fd` represents a typed memory object opened with
    /// the [`POSIX_TYPED_MEM_ALLOCATE_CONTIG`] flag, these allocated bytes shall be contiguous
    /// within the typed memory object. If `fd` represents a typed memory object opened with the
    /// [`POSIX_TYPED_MEM_ALLOCATE`] flag, these allocated bytes may be composed of non-contiguous
    /// fragments within the typed memory object. If `fd` represents a typed memory object opened
    /// with neither the [`POSIX_TYPED_MEM_ALLOCATE_CONTIG`] flag nor the
    /// [`POSIX_TYPED_MEM_ALLOCATE`] flag, `length` bytes starting at offset `offset` within the
    /// typed memory object are mapped, exactly as when mapping a file or shared memory object. In
    /// this case, if two processes map an area of typed memory using the same `offset` and
    /// `length` values and using file descriptors that refer to the same memory pool (either from
    /// the same port or from a different port), both processes shall map the same region of
    /// storage.
    ///
    /// When [`MAP_FIXED`] is set in the `flags` argument, the implementation is informed that the
    /// value returned shall be `addr`, exactly. If [`MAP_FIXED`] is set, [`mmap`] may return
    /// [`MAP_FAILED`] and set `errno` to [`EINVAL`]. If a [`MAP_FIXED`] request is successful, the
    /// mapping established by [`mmap`] replaces any previous mappings for the process' pages in
    /// the range `[ret, ret + length)`.
    ///
    /// When [`MAP_FIXED`] is not set, the implementation uses `addr` in an implementation-defined
    /// manner to arrive at the return value. The return value so chosen shall be an area of the
    /// address space that the implementation deems suitable for a mapping of `length` bytes to the
    /// file. All implementations interpret an `addr` value of 0 as granting the implementation
    /// complete freedom in selecting the return value, subject to constraints described below. A
    /// non-zero value of `addr` is taken to be a suggestion of a process address near which the
    /// mapping should be placed. When the implementation selects a value for the return value, it
    /// never places a mapping at address 0, nor does it replace any extant mapping.
    ///
    /// The `offset` argument is constrained to be aligned and sized according to the value
    /// returned by [`sysconf`] when passed [`_SC_PAGESIZE`] or [`_SC_PAGE_SIZE`]. When
    /// [`MAP_FIXED`] is specified, the application shall ensure that the argument `addr` also
    /// meets these constraints. The implementation performs mapping operations over whole pages.
    /// Thus, while the argument `length` need not meet a size or alignment constraint, the
    /// implementation shall include, in any mapping operation, any partial page specified by the
    /// range `[ret, ret + length)`.
    ///
    /// The system shall always zero-fill any partial page at the end of an object. Further, the
    /// system shall never write out any modified portions of the last page of an object which are
    /// beyond its end. References within the address range starting at the return value and
    /// continuing for `length` bytes to whole pages following the end of an object shall result in
    /// delivery of a [`SIGBUS`] signal.
    ///
    /// An implementation may generate [`SIGBUS`] signals when a reference would cause an error in
    /// the mapped object, such as out-of-space condition.
    ///
    /// The [`mmap`] function shall add an extra reference to the file associated with the file
    /// descriptor `fd` which is not removed by a subsequent [`close`] on that file descriptor.
    /// This reference shall be removed when there are no more mappings to the file.
    ///
    /// The `atime` field of the mapped file may be marked for update at any time between the
    /// [`mmap`] call and the corresponding [`munmap`] call. The initial read or write reference to
    /// a mapped region shall cause the file's `atime` field to be marked for update if it has not
    /// already been marked for update.
    ///
    /// The `ctime` and `mtime` fields of a file that is mapped with [`MAP_SHARED`] and
    /// [`PROT_WRITE`] shall be marked for update at some point in the interval between a write
    /// reference to the mapped region and the next call to [`msync`] with [`MS_ASYNC`] or
    /// [`MS_SYNC`] for that portion of the file by any process. If there is no such call and if
    /// the underlying file is modified as a result of a write reference, then these fields shall
    /// be marked for update at some time after the write reference.
    ///
    /// There may be implementation-defined limits on the number of memory regions that can be
    /// mapped (per process or per system).
    ///
    /// If such a limit is imposed, whether the number of memory regions that can be mapped by a
    /// process is decreased by the use of [`shmat`] is implementation-defined.
    ///
    /// If [`mmap`] fails for reasons other than [`EBADF`], [`EINVAL`], or [`ENOTSUP`], some of the
    /// mappings in the address range starting at `addr` and continuing for `length` bytes may have
    /// been unmapped.
    ///
    /// # Return Value
    /// Upon successful completion, the [`mmap`] function shall return the address at which the
    /// mapping was placed; otherwise, it shall return a value of [`MAP_FAILED`] and set errno to
    /// indicate the error. No successful return from [`mmap`] shall return the value
    /// [`MAP_FAILED`].
    pub fn mmap(
        addr: *mut c_void,
        length: c_size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
}
