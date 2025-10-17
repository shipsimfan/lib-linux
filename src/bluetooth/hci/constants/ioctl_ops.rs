use crate::{_IOR, _IOW};
use std::ffi::{c_int, c_ulong};

#[allow(missing_docs)]
pub const HCIDEVUP: c_ulong = _IOW!('H', 201, c_int);

#[allow(missing_docs)]
pub const HCIDEVDOWN: c_ulong = _IOW!('H', 202, c_int);

#[allow(missing_docs)]
pub const HCIDEVRESET: c_ulong = _IOW!('H', 203, c_int);

#[allow(missing_docs)]
pub const HCIDEVRESTAT: c_ulong = _IOW!('H', 204, c_int);

#[allow(missing_docs)]
pub const HCIGETDEVLIST: c_ulong = _IOR!('H', 210, c_int);

#[allow(missing_docs)]
pub const HCIGETDEVINFO: c_ulong = _IOR!('H', 211, c_int);

#[allow(missing_docs)]
pub const HCIGETCONNLIST: c_ulong = _IOR!('H', 212, c_int);

#[allow(missing_docs)]
pub const HCIGETCONNINFO: c_ulong = _IOR!('H', 213, c_int);

#[allow(missing_docs)]
pub const HCIGETAUTHINFO: c_ulong = _IOR!('H', 215, c_int);

#[allow(missing_docs)]
pub const HCISETRAW: c_ulong = _IOW!('H', 220, c_int);

#[allow(missing_docs)]
pub const HCISETSCAN: c_ulong = _IOW!('H', 221, c_int);

#[allow(missing_docs)]
pub const HCISETAUTH: c_ulong = _IOW!('H', 222, c_int);

#[allow(missing_docs)]
pub const HCISETENCRYPT: c_ulong = _IOW!('H', 223, c_int);

#[allow(missing_docs)]
pub const HCISETPTYPE: c_ulong = _IOW!('H', 224, c_int);

#[allow(missing_docs)]
pub const HCISETLINKPOL: c_ulong = _IOW!('H', 225, c_int);

#[allow(missing_docs)]
pub const HCISETLINKMODE: c_ulong = _IOW!('H', 226, c_int);

#[allow(missing_docs)]
pub const HCISETACLMTU: c_ulong = _IOW!('H', 227, c_int);

#[allow(missing_docs)]
pub const HCISETSCOMTU: c_ulong = _IOW!('H', 228, c_int);

#[allow(missing_docs)]
pub const HCIBLOCKADDR: c_ulong = _IOW!('H', 230, c_int);

#[allow(missing_docs)]
pub const HCIUNBLOCKADDR: c_ulong = _IOW!('H', 231, c_int);

#[allow(missing_docs)]
pub const HCIINQUIRY: c_ulong = _IOR!('H', 240, c_int);
