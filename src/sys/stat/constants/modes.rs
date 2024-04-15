/// Mask for the file type bit field
pub const S_IFMT: u16 = 0o0170000;

/// Socket
pub const S_IFSOCK: u16 = 0o140000;

/// Symbolic link
pub const S_IFLNK: u16 = 0o120000;

/// Regular file
pub const S_IFREG: u16 = 0o100000;

/// Block device
pub const S_IFBLK: u16 = 0o060000;

/// Directory
pub const S_IFDIR: u16 = 0o040000;

/// Character device
pub const S_IFCHR: u16 = 0o020000;

/// FIFO
pub const S_IFIFO: u16 = 0o010000;

/// Set user ID
pub const S_ISUID: u16 = 0o004000;

/// Set group ID
pub const S_ISGID: u16 = 0o002000;

/// Sticky bit
pub const S_ISVTX: u16 = 0o001000;

/// Owner read, write, and execute permissions
pub const S_IRWXU: u16 = 0o0700;

/// Owner read permission
pub const S_IRUSR: u16 = 0o0400;

/// Owner write permission
pub const S_IWUSR: u16 = 0o0200;

/// Owner execute permission
pub const S_IXUSR: u16 = 0o0100;

/// Group read, write, and execute permissions
pub const S_IRWXG: u16 = 0o0070;

/// Group read permissions
pub const S_IRGRP: u16 = 0o0040;

/// Group write permissions
pub const S_IWGRP: u16 = 0o0020;

/// Group execute permissions
pub const S_IXGRP: u16 = 0o0010;

/// Other read, write, and execute permissions
pub const S_IRWXO: u16 = 0o0007;

/// Other read permission
pub const S_IROTH: u16 = 0o0004;

/// Other write permission
pub const S_IWOTH: u16 = 0o0002;

/// Other execute permission
pub const S_IXOTH: u16 = 0o0001;
