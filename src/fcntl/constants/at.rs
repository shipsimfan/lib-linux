use std::ffi::c_int;

/// Special value used to indicate the *at functions should use the current working directory.
pub const AT_FDCWD: c_int = -100;

/// Do not follow symbolic links.
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;

/// Remove directory instead of unlinking file.
pub const AT_REMOVEDIR: c_int = 0x200;

/// Follow symbolic links.
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;

/// Suppress terminal automount traversal.
pub const AT_NO_AUTOMOUNT: c_int = 0x800;

/// Allow empty relative pathname.
pub const AT_EMPTY_PATH: c_int = 0x1000;

/// Do whatever [`stat`] does.    
pub const AT_STATX_SYNC_AS_STAT: c_int = 0x0000;

/// Force the attributes to be synchronized with the server.
pub const AT_STATX_FORCE_SYNC: c_int = 0x2000;

/// Don't synchronize anything.
pub const AT_STATX_DONT_SYNC: c_int = 0x4000;

/// Apply to the entire subtree.
pub const AT_RECURSIVE: c_int = 0x8000;
