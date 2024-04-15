use crate::sys::stat::statx_timestamp;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::stat::statx;

/// Structures for the extended file attribute retrieval system call ([`statx`]).
///
/// The caller passes a mask of what they're specifically interested in as a parameter to
/// [`statx`]. What [`statx`] actually got will be indicated in `mask` upon return.
///
/// For each bit in the mask argument:
///  - if the datum is not supported:
///    - the bit will be cleared, and
///    - the datum will be set to an appropriate fabricated value if one is available (eg. CIFS can
///      take a default uid and gid), otherwise
///    - the field will be cleared;
///  - otherwise, if explicitly requested:
///    - the datum will be synchronised to the server if [`AT_STATX_FORCE_SYNC`] is set or if the
///      datum is considered out of date, and
///    - the field will be filled in and the bit will be set;
///  - otherwise, if not requested, but available in approximate form without any effort, it will
///    be filled in anyway, and the bit will be set upon return (it might not be up to date,
///    however, and no attempt will be made to synchronise the internal state first);
///  - otherwise the field and the bit will be cleared before returning.
///
/// Items in [`STATX_BASIC_STATS`] may be marked unavailable on return, but they will have values
/// installed for compatibility purposes so that [`stat`] and co. can be emulated in userspace.
#[repr(C)]
pub struct Statx {
    /// What results were written
    pub mask: u32,

    /// Preferred general I/O size
    pub blksize: u32,

    /// Flags conveying information about the file
    pub attributes: u64,

    /// Number of hard links
    pub nlink: u32,

    /// User ID of owner
    pub uid: u32,

    /// Group ID of owner
    pub gid: u32,

    /// File mode
    pub mode: u16,

    /// Padding
    pub spare0: [u16; 1],

    /// Inode number
    pub ino: u64,

    /// File size
    pub size: u64,

    /// Number of 512-byte blocks allocated
    pub blocks: u64,

    /// Mask to show what's supported in `attributes`
    pub attributes_mask: u64,

    /// Last access time
    pub atime: statx_timestamp,

    /// File creation time
    pub btime: statx_timestamp,

    /// Last attribute change time
    pub ctime: statx_timestamp,

    /// Last data modification time
    pub mtime: statx_timestamp,

    /// Major number of the device ID of special file
    pub rdev_major: u32,

    /// Minor number of the device ID of special file
    pub rdev_minor: u32,

    /// Major number of the ID of the device containing file
    pub dev_major: u32,

    /// Minor number of the ID of the device containing file
    pub dev_minor: u32,

    /// ID of the mount
    pub mnt_id: u64,

    /// Memory buffer alignment for direct I/O
    pub dio_mem_align: u32,

    /// File offset alignment for direct I/O
    pub dio_offset_align: u32,

    /// Spare space for future expansion
    pub spare3: [u64; 12],
}

impl Default for Statx {
    fn default() -> Self {
        Statx {
            mask: 0,
            blksize: 0,
            attributes: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            mode: 0,
            spare0: [0],
            ino: 0,
            size: 0,
            blocks: 0,
            attributes_mask: 0,
            atime: statx_timestamp::default(),
            btime: statx_timestamp::default(),
            ctime: statx_timestamp::default(),
            mtime: statx_timestamp::default(),
            rdev_major: 0,
            rdev_minor: 0,
            dev_major: 0,
            dev_minor: 0,
            mnt_id: 0,
            dio_mem_align: 0,
            dio_offset_align: 0,
            spare3: [0; 12],
        }
    }
}
