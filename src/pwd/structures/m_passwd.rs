use crate::sys::types::{gid_t, uid_t};
use std::{ffi::c_char, ptr::null_mut};

/// A record in the user database.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct passwd {
    /// Username
    pub name: *mut c_char,

    /// Hashed passphrase, if shadow database not in use
    pub passwd: *mut c_char,

    /// User ID
    pub uid: uid_t,

    /// Group ID
    pub gid: gid_t,

    /// Real name
    pub gecos: *mut c_char,

    /// Home directory
    pub dir: *mut c_char,

    /// Shell program
    pub shell: *mut c_char,
}

impl Default for passwd {
    fn default() -> Self {
        passwd {
            name: null_mut(),
            passwd: null_mut(),
            uid: 0,
            gid: 0,
            gecos: null_mut(),
            dir: null_mut(),
            shell: null_mut(),
        }
    }
}
