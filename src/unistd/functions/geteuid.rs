use crate::sys::types::uid_t;

extern "C" {
    /// [`geteuid`] returns the effective user ID of the calling process.
    pub fn geteuid() -> uid_t;
}
