use std::ffi::c_ulong;

#[allow(missing_docs)]
pub const _IOC_NRBITS: c_ulong = 8;

#[allow(missing_docs)]
pub const _IOC_TYPEBITS: c_ulong = 8;

#[allow(missing_docs)]
pub const _IOC_SIZEBITS: c_ulong = 14;

#[allow(missing_docs)]
pub const _IOC_NRSHIFT: c_ulong = 0;

#[allow(missing_docs)]
pub const _IOC_TYPESHIFT: c_ulong = _IOC_NRSHIFT + _IOC_NRBITS;

#[allow(missing_docs)]
pub const _IOC_SIZESHIFT: c_ulong = _IOC_TYPESHIFT + _IOC_TYPEBITS;

#[allow(missing_docs)]
pub const _IOC_DIRSHIFT: c_ulong = _IOC_SIZESHIFT + _IOC_SIZEBITS;
