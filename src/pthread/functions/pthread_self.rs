use crate::pthread::pthread_t;

#[link(name = "pthread")]
extern "C" {
    /// Obtain ID of the calling thread
    ///
    /// The [`pthread_self`] function returns the ID of the calling thread. This is the same value
    /// that is returned in `*thread` in the [`pthread_create`] call that created this thread.
    ///
    /// # Return Value
    /// This function always succeeds, returning the calling thread's ID.
    pub fn pthread_self() -> pthread_t;
}
