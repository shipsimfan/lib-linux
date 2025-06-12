use std::ffi::{c_char, c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::dlfcn::{
    dlclose, dlerror, dlsym, RTLD_DEEPBIND, RTLD_GLOBAL, RTLD_LAZY, RTLD_LOCAL, RTLD_NODELETE,
    RTLD_NOLOAD, RTLD_NOW,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Open a shared object
    ///
    /// The function [`dlopen`] loads the dynamic shared object (shared library) file named by the
    /// null-terminated string `filename` and returns an opaque "handle" for the loaded object.
    /// This handle is employed with other functions in the [`dlopen`] API, such as [`dlsym`],
    /// [`dladdr`], [`dlinfo`], and [`dlclose`].
    ///
    /// If `filename` is [`null_mut`], then the returned handle is for the main program. If
    /// `filename` contains a slash ("/"), then it is interpreted as a (relative or absolute)
    /// pathname. Otherwise, the dynamic linker searches for the object as follows:
    ///  - (ELF only) If the calling object (i.e., the shared library or executable from which
    ///    [`dlopen`] is called) contains a `DT_RPATH` tag, and does not contain a `DT_RUNPATH`
    ///    tag, then the directories listed in the `DT_RPATH` tag are searched.
    ///  - If, at the time that the program was started, the environment variable `LD_LIBRARY_PATH`
    ///    was defined to contain a colon-separated list of directories, then these are searched.
    ///    (As a security measure, this variable is ignored for set-user-ID and set-group-ID
    ///    programs.)
    ///  - (ELF only) If the calling object contains a `DT_RUNPATH` tag, then the directories
    ///    listed in that tag are searched.
    ///  - The cache file /etc/ld.so.cache is checked to see whether it contains an entry for
    ///    filename.
    ///  - The directories /lib and /usr/lib are searched (in that order).
    ///
    /// If the object specified by `filename` has dependencies on other shared objects, then these
    /// are also automatically loaded by the dynamic linker using the same rules. (This process may
    /// occur recursively, if those objects in turn have dependencies, and so on.)
    ///
    /// One of the following two values must be included in `flags`:
    ///  * [`RTLD_LAZY`] - Perform lazy binding. Resolve symbols only as the code that references
    ///                    them is executed. If the symbol is never referenced, then it is never
    ///                    resolved. (Lazy binding is performed only for function references;
    ///                    references to variables are always immediately bound when the shared
    ///                    object is loaded.) Since glibc 2.1.1, this flag is overridden by the
    ///                    effect of the `LD_BIND_NOW` environment variable.
    ///  * [`RTLD_NOW`] - If this value is specified, or the environment variable `LD_BIND_NOW` is
    ///                   set to a nonempty string, all undefined symbols in the shared object are
    ///                   resolved before [`dlopen`] returns. If this cannot be done, an error is
    ///                   returned.
    ///
    /// Zero or more of the following values may also be ORed in `flags`:
    ///  * [`RTLD_GLOBAL`] - The symbols defined by this shared object will be made available for
    ///                      symbol resolution of subsequently loaded shared objects.
    ///  * [`RTLD_LOCAL`] - This is the converse of [`RTLD_GLOBAL`], and the default if neither
    ///                     flag is specified. Symbols defined in this shared object are not made
    ///                     available to resolve references in subsequently loaded shared objects.
    ///  * [`RTLD_NODELETE`] - (since glibc 2.2) Do not unload the shared object during
    ///                        [`dlclose`]. Consequently, the object's static and global variables
    ///                        are not reinitialized if the object is reloaded with [`dlopen`] at a
    ///                        later time.
    ///  * [`RTLD_NOLOAD`] - (since glibc 2.2) Don't load the shared object. This can be used to
    ///                      test if the object is already resident ([`dlopen`] returns
    ///                      [`null_mut`] if it is not, or the object's handle if it is resident).
    ///                      This flag can also be used to promote the flags on a shared object
    ///                      that is already loaded.  For example, a shared object that was
    ///                      previously loaded with [`RTLD_LOCAL`] can be reopened with
    ///                      [`RTLD_NOLOAD`] | [`RTLD_GLOBAL`].
    ///  * [`RTLD_DEEPBIND`] - (since glibc 2.3.4) Place the lookup scope of the symbols in this
    ///                        shared object ahead of the global scope. This means that a self-
    ///                        contained object will use its own symbols in preference to global
    ///                        symbols with the same name contained in objects that have already
    ///                        been loaded.
    ///
    /// If `filename` is [`null_mut`], then the returned handle is for the main program. When
    /// given to [`dlsym`], this handle causes a search for a symbol in the main program, followed
    /// by all shared objects loaded at program startup, and then all shared objects loaded by
    /// [`dlopen`] with the flag [`RTLD_GLOBAL`].
    ///
    /// Symbol references in the shared object are resolved using (in order): symbols in the link
    /// map of objects loaded for the main program and its dependencies; symbols in shared objects
    /// (and their dependencies) that were previously opened with [`dlopen`] using the
    /// [`RTLD_GLOBAL`] flag; and definitions in the shared object itself (and any dependencies
    /// that were loaded for that object).
    ///
    /// Any global symbols in the executable that were placed into its dynamic symbol table by ld
    /// can also be used to resolve references in a dynamically loaded shared object. Symbols may
    /// be placed in the dynamic symbol table either because the executable was linked with the
    /// flag "-rdynamic" (or, synonymously, "--export-dynamic"), which causes all of the
    /// executable's global symbols to be placed in the dynamic symbol table, or because ld noted a
    /// dependency on a symbol in another object during static linking.
    ///
    /// If the same shared object is opened again with [`dlopen`], the same object handle is
    /// returned. The dynamic linker maintains reference counts for object handles, so a
    /// dynamically loaded shared object is not deallocated until [`dlclose`] has been called on it
    /// as many times as [`dlopen`] has succeeded on it. Constructors (see below) are called only
    /// when the object is actually loaded into memory (i.e., when the reference count increases to
    /// 1).
    ///
    /// A subsequent [`dlopen`] call that loads the same shared object with [`RTLD_NOW`] may force
    /// symbol resolution for a shared object earlier loaded with [`RTLD_LAZY`]. Similarly, an
    /// object that was previously opened with [`RTLD_LOCAL`] can be promoted to [`RTLD_GLOBAL`] in
    /// a subsequent [`dlopen`].
    ///
    /// If [`dlopen`] fails for any reason, it returns [`null_mut`].
    ///
    /// # Return Value
    /// On success, [`dlopen`] returns a non-[`null_mut`] handle for the loaded object. On error
    /// (file could not be found, was not readable, had the wrong format, or caused errors during
    /// loading), [`dlopen`] returns [`null_mut`].
    ///
    /// Errors from [`dlopen`] can be diagnosed using [`dlerror`].
    pub fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
}
