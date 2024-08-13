use std::ffi::c_char;

extern "C" {
    #[allow(missing_docs)]
    pub static tzname: [*const c_char; 2];
}
