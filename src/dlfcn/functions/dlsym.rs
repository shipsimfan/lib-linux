use std::ffi::{c_char, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::dlfcn::{dlerror, dlopen, RTLD_DEFAULT, RTLD_GLOBAL, RTLD_NEXT};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Obtain address of a symbol in a shared object or executable
    ///
    /// The function [`dlsym`] takes a "handle" of a dynamic loaded shared object returned by
    /// [`dlopen`] along with a null-terminated symbol name, and returns the address where that
    /// symbol is loaded into memory. If the symbol is not found, in the specified object or any of
    /// the shared objects that were automatically loaded by [`dlopen`] when that object was
    /// loaded, [`dlsym`] returns [`null_mut`]. (The search performed by [`dlsym`] is breadth first
    /// through the dependency tree of these shared objects.)
    ///
    /// In unusual cases the value of the symbol could actually be [`null_mut`]. Therefore, a
    /// [`null_mut`] return from [`dlsym`] need not indicate an error. The correct way to
    /// distinguish an error from a symbol whose value is [`null_mut`] is to call [`dlerror`] to
    /// clear any old error conditions, then call [`dlsym`], and then call [`dlerror`] again,
    /// saving its return value into a variable, and check whether this saved value is not
    /// [`null_mut`].
    ///
    /// There are two special pseudo-handles that may be specified in handle:
    ///  * [`RTLD_DEFAULT`] - Find the first occurrence of the desired symbol using the default
    ///                       shared object search order. The search will include global symbols in
    ///                       the executable and its dependencies, as well as symbols in shared
    ///                       objects that were dynamically loaded with the [`RTLD_GLOBAL`] flag.
    ///  * [`RTLD_NEXT`] - Find the next occurrence of the desired symbol in the search order after
    ///                    the current object. This allows one to provide a wrapper around a
    ///                    function in another shared object, so that, for example, the definition
    ///                    of a function in a preloaded shared object can find and invoke the
    ///                    "real" function provided in another shared object (or for that matter,
    ///                    the "next" definition of the function in cases where there are multiple
    ///                    layers of preloading).
    ///
    /// # Return Value
    /// On success, [`dlsym`] returns the address associated with `symbol`. On failure, they return
    /// [`null_mut`]; the error can be diagnosed using [`dlerror`].
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}
