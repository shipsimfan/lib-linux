use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::ETXTBSY;

/// Use a file.
pub const MAP_FILE: c_int = 0x00;

/// Share changes.
pub const MAP_SHARED: c_int = 0x01;

/// Changes are private.
pub const MAP_PRIVATE: c_int = 0x02;

/// Share changes, but validate.
pub const MAP_SHARED_VALIDATE: c_int = 0x03;

/// Mask for type of mapping.
pub const MAP_TYPE: c_int = 0x0F;

/// Interpret addr exactly.
pub const MAP_FIXED: c_int = 0x10;

/// Don't use a file.
pub const MAP_ANONYMOUS: c_int = 0x20;

/// Don't use a file.
pub const MAP_ANON: c_int = MAP_ANONYMOUS;

/// Only give out 32-bit addresses.
pub const MAP_32BIT: c_int = 0x40;

/// Stack-like segment.
pub const MAP_GROWSDOWN: c_int = 0x00100;

/// [`ETXTBSY`]
pub const MAP_DENYWRITE: c_int = 0x00800;

/// Mark it as an executable.
pub const MAP_EXECUTABLE: c_int = 0x01000;

/// Lock the mapping.
pub const MAP_LOCKED: c_int = 0x02000;

/// Do not block on IO.
pub const MAP_NONBLOCK: c_int = 0x10000;

/// Create huge page mapping.
pub const MAP_HUGETLB: c_int = 0x40000;

/// [`MAP_FIXED`] but do not unmap underlying mapping.
pub const MAP_FIXED_NOREPLACE: c_int = 0x100000;

/// Don't check for reservations.
pub const MAP_NORESERVE: c_int = 0x04000;

/// Populate (prefault) pagetables.
pub const MAP_POPULATE: c_int = 0x08000;

/// Allocation is for a stack.
pub const MAP_STACK: c_int = 0x20000;

/// Perform synchronous page faults for the mapping.
pub const MAP_SYNC: c_int = 0x80000;
