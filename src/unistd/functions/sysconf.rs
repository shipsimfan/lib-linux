use std::ffi::{c_int, c_long};

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{EINVAL, errno};

unsafe extern "C" {
    /// Get configuration information at run time
    ///
    /// # Description
    /// POSIX allows an application to test at compile or run time whether certain options are
    /// supported, or what the value is of certain configurable constants or limits.
    ///
    /// At compile time this is done by including <unistd.h> and/or <limits.h> and testing the
    /// value of certain macros.
    ///
    /// At run time, one can ask for numerical values using the present function [`sysconf`]. One
    /// can ask for numerical values that may depend on the filesystem in which a file resides
    /// using [`fpathconf`] and [`pathconf`]. One can ask for string values using [`confstr`].
    ///
    /// The values obtained from these functions are system configuration constants. They do not
    /// change during the lifetime of a process.
    ///
    /// For options, typically, there is a constant `_POSIX_FOO` that may be defined in <unistd.h>.
    /// If it is undefined, one should ask at run time. If it is defined to -1, then the option is
    /// not supported. If it is defined to 0, then relevant functions and headers exist, but one
    /// has to ask at run time what degree of support is available. If it is defined to a value
    /// other than -1 or 0, then the option is supported. Usually the value (such as 200112L)
    /// indicates the year and month of the POSIX revision describing the option. glibc uses the
    /// value 1 to indicate support as long as the POSIX revision has not been published yet. The
    /// [`sysconf`] argument will be `_SC_FOO`.
    ///
    /// For variables or limits, typically, there is a constant `_FOO`, maybe defined in
    /// <limits.h>, or `_POSIX_FOO`, maybe defined in <unistd.h>. The constant will not be defined
    /// if the limit is unspecified. If the constant is defined, it gives a guaranteed value, and a
    /// greater value might actually be supported. If an application wants to take advantage of
    /// values which may change between systems, a call to [`sysconf`] can be made. The [`sysconf`]
    /// argument will be `_SC_FOO`.
    ///
    /// # Return Value
    /// If `name` is invalid, -1 is returned, and [`errno`] is set to [`EINVAL`]. Otherwise, the
    /// value returned is the value of the system resource and errno is not changed. In the case of
    /// options, a positive value is returned if a queried option is available, and -1 if it is
    /// not. In the case of limits, -1 means that there is no definite limit.
    pub fn sysconf(name: c_int) -> c_long;
}
