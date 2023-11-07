use crate::raw::locale_t;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::newlocale;

extern "C" {
    /// freelocale - free a locale object
    ///
    /// ## Description
    /// The [`freelocale`] function deallocates the resources associated with `locobj`, a locale
    /// object previously returned by a call to [`newlocale`] or [`duplocale`]. If `locobj` is
    /// [`LC_GLOBAL_LOCALE`] or is not valid locale object handle, the results are undefined.
    ///
    /// Once a locale object has been freed, the program should make no further use of it.
    pub fn freelocale(locobj: locale_t);
}
