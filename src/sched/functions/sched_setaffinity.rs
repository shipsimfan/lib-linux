use crate::{c_size_t, sched::cpu_set_t, unistd::pid_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{EFAULT, EINVAL, EPERM, ESRCH, errno},
    linux::capability::CAP_SYS_NICE,
};

unsafe extern "C" {
    /// Set a thread's CPU affinity mask
    ///
    /// # Description
    /// A process's CPU affinity mask determines the set of CPUs on which it is eligible to run. On
    /// a multiprocessor system, setting the CPU affinity mask can be used to obtain performance
    /// benefits. For example, by dedicating one CPU to a particular process (i.e., setting the
    /// affinity mask of that process to specify a single CPU, and setting the affinity mask of all
    /// other processes to exclude that CPU), it is possible to ensure maximum execution speed for
    /// that process. Restricting a process to run on a single CPU also avoids the performance cost
    /// caused by the cache invalidation that occurs when a process ceases to execute on one CPU
    /// and then recommences execution on a different CPU.
    ///
    /// A CPU affinity mask is represented by the [`cpu_set_t`] structure, a "CPU set", pointed to
    /// by `mask`.
    ///
    /// [`sched_setaffinity`] sets the CPU affinity mask of the process whose ID is `pid` to the
    /// value specified by `mask`. If `pid` is zero, then the calling process is used. The argument
    /// `cpusetsize` is the length (in bytes) of the data pointed to by `mask`. Normally this
    /// argument would be specified as `std::mem::size_of::<cpu_set_t>()`.
    ///
    /// If the process specified by `pid` is not currently running on one of the CPUs specified in
    /// `mask`, then that process is migrated to one of the CPUs specified in `mask`.
    ///
    /// # Return Value
    /// On success, [`sched_setaffinity`] returns 0. On error, -1 is returned, and [`errno`] is set
    /// appropriately.
    ///
    /// # Errors
    ///  * [`EFAULT`] - A supplied memory address was invalid.
    ///  * [`EINVAL`] - The affinity bit mask mask contains no processors that are currently
    ///                 physically on the system and permitted to the process according to any
    ///                 restrictions that may be imposed by the "cpuset" mechanism.
    ///  * [`EPERM`] - The calling process does not have appropriate privileges. The caller needs
    ///                an effective user ID equal to the real user ID or effective user ID of the
    ///                process identified by `pid`, or it must possess the [`CAP_SYS_NICE`]
    ///                capability.
    ///  * [`ESRCH`] - The process whose ID is `pid` could not be found.
    pub fn sched_setaffinity(pid: pid_t, cpusetsize: c_size_t, mask: *const cpu_set_t) -> c_int;
}
