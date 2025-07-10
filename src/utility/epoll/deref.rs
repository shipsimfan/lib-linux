use crate::EPoll;
use std::{ffi::c_int, ops::Deref};

impl Deref for EPoll {
    type Target = c_int;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl AsRef<c_int> for EPoll {
    fn as_ref(&self) -> &c_int {
        &self.handle
    }
}
