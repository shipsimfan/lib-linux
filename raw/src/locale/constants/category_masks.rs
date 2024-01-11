use super::categories::*;
use std::ffi::c_int;

/// Mask for c types ([`LC_CTYPE`])
pub const LC_CTYPE_MASK: c_int = 1 << LC_CTYPE;

/// Mask for numeric ([`LC_NUMERIC`])
pub const LC_NUMERIC_MASK: c_int = 1 << LC_NUMERIC;

/// Mask for time ([`LC_TIME`])
pub const LC_TIME_MASK: c_int = 1 << LC_TIME;

/// Mask for collate ([`LC_COLLATE`])
pub const LC_COLLATE_MASK: c_int = 1 << LC_COLLATE;

/// Mask for monetary ([`LC_MONETARY`])
pub const LC_MONETARY_MASK: c_int = 1 << LC_MONETARY;

/// Mask for messages ([`LC_MESSAGES`])
pub const LC_MESSAGES_MASK: c_int = 1 << LC_MESSAGES;

/// Mask for all ([`LC_ALL`])
pub const LC_ALL_MASK: c_int = 1 << LC_ALL;

/// Mask for paper ([`LC_PAPER`])
pub const LC_PAPER_MASK: c_int = 1 << LC_PAPER;

/// Mask for name ([`LC_NAME`])
pub const LC_NAME_MASK: c_int = 1 << LC_NAME;

/// Mask for address ([`LC_ADDRESS`])
pub const LC_ADDRESS_MASK: c_int = 1 << LC_ADDRESS;

/// Mask for telephone ([`LC_TELEPHONE`])
pub const LC_TELEPHONE_MASK: c_int = 1 << LC_TELEPHONE;

/// Mask for measurement ([`LC_MEASUREMENT`])
pub const LC_MEASUREMENT_MASK: c_int = 1 << LC_MEASUREMENT;

/// Mask for identification ([`LC_IDENTIFICATION`])
pub const LC_IDENTIFICATION_MASK: c_int = 1 << LC_IDENTIFICATION;
