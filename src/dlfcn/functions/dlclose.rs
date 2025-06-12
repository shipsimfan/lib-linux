use std::{ffi::c_int, os::raw::c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::dlfcn::{dlerror, dlopen, RTLD_GLOBAL};

extern "C" {
    /// The function [`dlclose`] decrements the reference count on the dynamically loaded shared
    /// object referred to by `handle`.
    ///
    /// If the object's reference count drops to zero and no symbols in this object are required by
    /// other objects, then the object is unloaded after first calling any destructors defined for
    /// the object. (Symbols in this object might be required in another object because this object
    /// was opened with the [`RTLD_GLOBAL`] flag and one of its symbols satisfied a relocation in
    /// another object.)
    ///
    /// All shared objects that were automatically loaded when [`dlopen`] was invoked on the object
    /// referred to by `handle` are recursively closed in the same manner.
    ///
    /// A successful return from [`dlclose`] does not guarantee that the symbols associated with
    /// handle are removed from the caller's address space. In addition to references resulting
    /// from explicit [`dlopen`] calls, a shared object may have been implicitly loaded (and
    /// reference counted) because of dependencies in other shared objects. Only when all
    /// references have been released can the shared object be removed from the address space.
    ///
    /// # Return Value
    /// On success, [`dlclose`] returns 0; on error, it returns a nonzero value. Errors can be
    /// diagnosed using [`dlerror`].
    pub fn dlclose(handle: *mut c_void) -> c_int;
}
