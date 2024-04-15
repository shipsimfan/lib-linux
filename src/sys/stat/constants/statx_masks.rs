use std::ffi::c_uint;

/// Want/got `mode & S_IFMT`
pub const STATX_TYPE: c_uint = 0x00000001;

/// Want/got `mode & ~S_IFMT`
pub const STATX_MODE: c_uint = 0x00000002;

/// Want/got `nlink`
pub const STATX_NLINK: c_uint = 0x00000004;

/// Want/got `uid`
pub const STATX_UID: c_uint = 0x00000008;

/// Want/got `gid`
pub const STATX_GID: c_uint = 0x00000010;

/// Want/got `atime`
pub const STATX_ATIME: c_uint = 0x00000020;

/// Want/got `mtime`
pub const STATX_MTIME: c_uint = 0x00000040;

/// Want/got `ctime`
pub const STATX_CTIME: c_uint = 0x00000080;

/// Want/got `ino`
pub const STATX_INO: c_uint = 0x00000100;

/// Want/got `size`
pub const STATX_SIZE: c_uint = 0x00000200;

/// Want/got `blocks`
pub const STATX_BLOCKS: c_uint = 0x00000400;

/// The stuff in the normal [`stat`] struct
pub const STATX_BASIC_STATS: c_uint = 0x000007FF;

/// Want/got `btime`
pub const STATX_BTIME: c_uint = 0x00000800;

/// Got `mnt_id`
pub const STATX_MNT_ID: c_uint = 0x00001000;

/// Want/got direct I/O alignment info
pub const STATX_DIOALIGN: c_uint = 0x00002000;

/// Reserved for [`Statx`] expansion
#[allow(non_upper_case_globals)]
pub const STATX__RESERVED: c_uint = 0x80000000;
