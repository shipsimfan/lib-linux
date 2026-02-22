use std::ffi::c_int;

unsafe extern "C" {
    #[allow(missing_docs)]
    pub static daylight: c_int;
}
