use crate::locale::locale_t;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::errno,
    locale::{
        freelocale, LC_ADDRESS_MASK, LC_ALL_MASK, LC_COLLATE_MASK, LC_CTYPE_MASK,
        LC_IDENTIFICATION_MASK, LC_MEASUREMENT_MASK, LC_MESSAGES_MASK, LC_MONETARY_MASK,
        LC_NAME_MASK, LC_NUMERIC_MASK, LC_PAPER_MASK, LC_TELEPHONE_MASK, LC_TIME_MASK,
    },
};
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    /// newlocale - create and modify a locale object
    ///
    /// # Description
    /// The [`newlocale`] function creates a new locale object, or modifies an existing object,
    /// returning a reference to the new or modified object as the function result. Whether the
    /// call creates a new object or modifies an existing object is determined by the value of
    /// `base`:
    ///  -  If `base` is [`null`], a new object is created.
    ///  -  If `base` refers to valid existing locale object (i.e., an object returned by a
    ///     previous call to [`newlocale`] or [`duplocale`]), then that object is modified by the
    ///     call. If the call is successful, the contents of base are unspecified (in particular,
    ///     the object referred to by base may be freed, and a new object created). Therefore, the
    ///     caller should ensure that it stops using base before the call to [`newlocale`], and
    ///     should subsequently refer to the modified object via the reference returned as the
    ///     function result. If the call fails, the contents of base remain valid and unchanged.
    ///
    /// If base is the special locale object [`LC_GLOBAL_LOCALE`] (see [`duplocale`]), or is not
    /// [`null`] and is not a valid locale object handle, the behavior is undefined.
    ///
    /// The `category_mask` argument is a bit mask that specifies the locale categories that are to
    /// be set in a newly created locale object or modified in an existing object. The mask is
    /// constructed by a bitwise OR of the constants:
    ///  * [`LC_ADDRESS_MASK`]
    ///  * [`LC_CTYPE_MASK`]
    ///  * [`LC_COLLATE_MASK`]
    ///  * [`LC_IDENTIFICATION_MASK`]
    ///  * [`LC_MEASUREMENT_MASK`]
    ///  * [`LC_MESSAGES_MASK`]
    ///  * [`LC_MONETARY_MASK`]
    ///  * [`LC_NUMERIC_MASK`]
    ///  * [`LC_NAME_MASK`]
    ///  * [`LC_PAPER_MASK`]
    ///  * [`LC_TELEPHONE_MASK`]
    ///  * [`LC_TIME_MASK`]
    ///
    /// Alternatively, the mask can be specified as [`LC_ALL_MASK`], which is equivalent to ORing
    /// all of the preceding constants.
    ///
    /// For each category specified in `category_mask`, the locale data from locale will be used in
    /// the object returned by [`newlocale`]. If a new locale object is being created, data for all
    /// categories not specified in `category_mask` is taken from the default ("POSIX") locale.
    ///
    /// The following preset values of locale are defined for all categories that can be specified
    /// in `category_mask`:
    ///  * "POSIX" - A minimal locale environment for C language programs.
    ///  * "C" - Equivalent to "POSIX".
    ///  * "" - An implementation-defined native environment corresponding to the values of the
    ///         `LC_*` and `LANG` environment variables.
    ///
    /// # Return Value
    /// On success, [`newlocale`] returns a handle that can be used in calls to [`duplocale`],
    /// [`freelocale`], and other functions that take a [`locale_t`] argument.  On error,
    /// [`newlocale`] returns [`null`], and sets [`errno`] to indicate the error.
    pub fn newlocale(category_mask: c_int, locale: *const c_char, base: locale_t) -> locale_t;
}
