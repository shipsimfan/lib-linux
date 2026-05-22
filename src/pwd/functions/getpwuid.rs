use crate::{pwd::passwd, sys::types::uid_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::{errno::errno, stdlib::free};
#[allow(unused_imports)]
use std::ptr::null_mut;

unsafe extern "C" {
    /// Get password file entry
    ///
    /// # Description
    /// The [`getpwuid`] function returns a pointer to a structure containing the broken-out fields
    /// of the record in the password database that matches the user ID `uid`.
    ///
    /// # Return Value
    /// The [`getpwuid`] function returns a pointer to a [`passwd`] structure, or [`null_mut`] if
    /// the matching entry is not found or an error occurs. If an error occurs, [`errno`] is set
    /// appropriately. If one wants to check [`errno`] after the call, it should be set to zero
    /// before the call.
    ///
    /// The return value may point to a static area, and may be overwritten by subsequent calls to
    /// [`getpwent`], [`getpwnam`], or [`getpwuid`]. (Do not pass the returned pointer to
    /// [`free`].)
    pub fn getpwuid(uid: uid_t) -> *mut passwd;
}
