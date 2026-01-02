use crate::sys::utsname::{
    _UTSNAME_MACHINE_LENGTH, _UTSNAME_NODENAME_LENGTH, _UTSNAME_RELEASE_LENGTH,
    _UTSNAME_SYSNAME_LENGTH, _UTSNAME_VERSION_LENGTH,
};
use std::ffi::c_char;

/// Structure describing the system and machine
#[repr(C)]
#[derive(Debug, Clone)]
pub struct utsname {
    /// Name of the implementation of the operating system
    pub sysname: [c_char; _UTSNAME_SYSNAME_LENGTH],

    /// Name of this node on the network
    pub nodename: [c_char; _UTSNAME_NODENAME_LENGTH],

    /// Current release level of this implementation
    pub release: [c_char; _UTSNAME_RELEASE_LENGTH],

    /// Current version level of this release
    pub version: [c_char; _UTSNAME_VERSION_LENGTH],

    /// Name of the hardware type the system is running on
    pub machine: [c_char; _UTSNAME_MACHINE_LENGTH],
}
