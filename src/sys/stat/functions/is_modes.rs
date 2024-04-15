use crate::sys::stat::{S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK};

/// Is this `mode` a symbolic link
#[inline(always)]
pub const fn s_islnk(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFLNK
}

/// Is this `mode` a regular file
#[inline(always)]
pub const fn s_isreg(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFREG
}

/// Is this `mode` a directory
#[inline(always)]
pub const fn s_isdir(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

/// Is this `mode` a character device
#[inline(always)]
pub const fn s_ischr(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFCHR
}

/// Is this `mode` a block device
#[inline(always)]
pub const fn s_isblk(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFBLK
}

/// Is this `mode` a FIFO
#[inline(always)]
pub const fn s_isfifo(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFIFO
}

/// Is this `mode` a socket
#[inline(always)]
pub const fn s_issock(mode: u16) -> bool {
    (mode & S_IFMT) == S_IFSOCK
}
