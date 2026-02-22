use std::ffi::c_long;

unsafe extern "C" {
    #[allow(missing_docs)]
    pub static timezone: c_long;
}
