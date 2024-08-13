use std::ffi::c_long;

extern "C" {
    #[allow(missing_docs)]
    pub static timezone: c_long;
}
