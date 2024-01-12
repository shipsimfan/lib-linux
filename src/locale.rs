use crate::{Error, Result};
use raw::locale::{freelocale, locale_t, newlocale};
use std::{ffi::CStr, ptr::null_mut};

pub use crate::raw::locale::{
    LC_ADDRESS_MASK, LC_ALL_MASK, LC_COLLATE_MASK, LC_CTYPE_MASK, LC_IDENTIFICATION_MASK,
    LC_MEASUREMENT_MASK, LC_MESSAGES_MASK, LC_MONETARY_MASK, LC_NAME_MASK, LC_NUMERIC_MASK,
    LC_PAPER_MASK, LC_TELEPHONE_MASK, LC_TIME_MASK,
};

/// A locale object
pub struct Locale {
    inner: locale_t,
}

impl Locale {
    /// Creates a new [`Locale`]
    ///
    /// # Parameters
    ///  * `category_mask` - A bitwise OR of `LC_*_MASK` values
    ///  * `locale` - A string representing the requested locale
    ///  * `base` - A locale to extend from
    ///
    /// # Return Value
    /// Returns a new [`Locale`] on success or the error that occurred while trying to create it.
    ///
    /// # Remarks
    /// See [`newlocale`] for more information on this function
    pub fn new(category_mask: i32, locale: &CStr, base: Option<&Locale>) -> Result<Self> {
        let inner = unsafe {
            newlocale(
                category_mask,
                locale.as_ptr(),
                base.map(|locale| locale.inner()).unwrap_or(null_mut()),
            )
        };

        if inner.is_null() {
            Err(Error::errno())
        } else {
            Ok(Locale { inner })
        }
    }

    /// Returns the underlying locale pointer
    pub unsafe fn inner(&self) -> locale_t {
        self.inner
    }
}

impl Drop for Locale {
    fn drop(&mut self) {
        unsafe { freelocale(self.inner) };
    }
}
