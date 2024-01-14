use crate::sys::types::uid_t;

extern "C" {
    /// [`getuid`] returns the real user ID of the calling process.
    pub fn getuid() -> uid_t;
}
