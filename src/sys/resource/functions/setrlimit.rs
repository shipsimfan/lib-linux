use crate::sys::resource::rlimit;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{EAGAIN, EFAULT, EFBIG, EINVAL, EMFILE, ENOMEM, EPERM, ESRCH, errno},
    fcntl::fcntl,
    linux::capability::CAP_SYS_RESOURCE,
    signal::{SIGKILL, SIGSEGV, SIGXCPU, SIGXFSZ, kill},
    sys::{
        resource::{
            RLIM_INFINITY, RLIMIT_AS, RLIMIT_CORE, RLIMIT_CPU, RLIMIT_DATA, RLIMIT_FSIZE,
            RLIMIT_LOCKS, RLIMIT_MEMLOCK, RLIMIT_MSGQUEUE, RLIMIT_NICE, RLIMIT_NOFILE,
            RLIMIT_NPROC, RLIMIT_RSS, RLIMIT_RTPRIO, RLIMIT_RTTIME, RLIMIT_SIGPENDING,
            RLIMIT_STACK,
        },
        stat::open,
    },
    unistd::{pipe, write},
};

unsafe extern "C" {
    /// Set resource limits
    ///
    /// # Description
    /// The [`setrlimit`] system call sets resource limits. Each resource has an associated soft
    /// and hard limit, as defined by the [`rlimit`] structure.
    ///
    /// The soft limit is the value that the kernel enforces for the corresponding resource. The
    /// hard limit acts as a ceiling for the soft limit: an unprivileged process may only set its
    /// soft limit to a value in the range from 0 up to the hard limit, and (irreversibly) lower
    /// its hard limit. A privileged process (under Linux: one with the [`CAP_SYS_RESOURCE`]
    /// capability) may make arbitrary changes to either limit value.
    ///
    /// The value [`RLIM_INFINITY`] denotes no limit on a resource.
    ///
    /// The `resource` argument must be one of:
    ///  * [`RLIMIT_AS`] - The maximum size of the process's virtual memory (address space) in
    ///                    bytes. This limit affects calls to [`brk`], [`mmap`] and [`mremap`],
    ///                    which fail with the error [`ENOMEM`] upon exceeding this limit. Also
    ///                    automatic stack expansion will fail (and generate a [`SIGSEGV`] that
    ///                    kills the process if no alternate stack has been made available via
    ///                    [`sigaltstack`]). Since the value is a long, on machines with a 32-bit
    ///                    long either this limit is at most 2 GiB, or this resource is unlimited.
    ///  * [`RLIMIT_CORE`] - Maximum size of core file. When 0 no core dump files are created. When
    ///                      nonzero, larger dumps are truncated to this size.
    ///  * [`RLIMIT_CPU`] - CPU time limit in seconds. When the process reaches the soft limit, it
    ///                     is sent a [`SIGXCPU`] signal. The default action for this signal is to
    ///                     terminate the process. However, the signal can be caught, and the
    ///                     handler can return control to the main program. If the process
    ///                     continues to consume CPU time, it will be sent [`SIGXCPU`] once per
    ///                     second until the hard limit is reached, at which time it is sent
    ///                     [`SIGKILL`]. (This latter point describes Linux behavior.
    ///                     Implementations vary in how they treat processes which continue to
    ///                     consume CPU time after reaching the soft limit. Portable applications
    ///                     that need to catch this signal should perform an orderly termination
    ///                     upon first receipt of [`SIGXCPU`].)
    ///  * [`RLIMIT_DATA`] - The maximum size of the process's data segment (initialized data,
    ///                      uninitialized data, and heap). This limit affects calls to [`brk`] and
    ///                      [`sbrk`], which fail with the error [`ENOMEM`] upon encountering the
    ///                      soft limit of this resource.
    ///  * [`RLIMIT_FSIZE`] - The maximum size of files that the process may create. Attempts to
    ///                       extend a file beyond this limit result in delivery of a [`SIGXFSZ`]
    ///                       signal. By default, this signal terminates a process, but a process
    ///                       can catch this signal instead, in which case the relevant system call
    ///                       (e.g., [`write()`], [`truncate`]) fails with the error [`EFBIG`].
    ///  * [`RLIMIT_LOCKS`] (Early Linux 2.4 only) - A limit on the combined number of [`flock`]
    ///                                              locks and [`fcntl`] leases that this process
    ///                                              may establish.
    ///  * [`RLIMIT_MEMLOCK`] - The maximum number of bytes of memory that may be locked into RAM.
    ///                         In effect this limit is rounded down to the nearest multiple of the
    ///                         system page size. This limit affects [`mlock`] and [`mlockall`] and
    ///                         the [`mmap`] [`MAP_LOCKED`] operation. Since Linux 2.6.9 it also
    ///                         affects the [`shmctl`] [`SHM_LOCK`] operation, where it sets a
    ///                         maximum on the total bytes in shared memory segments (see
    ///                         [`shmget`]) that may be locked by the real user ID of the calling
    ///                         process. The [`shmctl`] [`SHM_LOCK`] locks are accounted for
    ///                         separately from the per-process memory locks established by
    ///                         [`mlock`], [`mlockall`], and [`mmap`] [`MAP_LOCKED`]; a process can
    ///                         lock bytes up to this limit in each of these two categories. In
    ///                         Linux kernels before 2.6.9, this limit controlled the amount of
    ///                         memory that could be locked by a privileged process. Since Linux
    ///                         2.6.9, no limits are placed on the amount of memory that a
    ///                         privileged process may lock, and this limit instead governs the
    ///                         amount of memory that an unprivileged process may lock.
    ///  * [`RLIMIT_MSGQUEUE`] (Since Linux 2.6.8) - Specifies the limit on the number of bytes
    ///                                              that can be allocated for POSIX message queues
    ///                                              for the real user ID of the calling process.
    ///                                              This limit is enforced for [`mq_open`]. Each
    ///                                              message queue that the user creates counts
    ///                                              (until it is removed) against this limit
    ///                                              according to the formula: `bytes =
    ///                                              attr.mq_maxmsg *
    ///                                              std::mem::size_of::<*mut msg_msg>() +
    ///                                              attr.mq_maxmsg * attr.mq_msgsize` where `attr`
    ///                                              is the [`mq_attr`] structure specified as the
    ///                                              fourth argument to [`mq_open`] The first
    ///                                              addend in the formula, which includes
    ///                                              `std::mem::size_of::<*mut msg_msg>()` (4 bytes
    ///                                              on Linux/i386), ensures that the user cannot
    ///                                              create an unlimited number of zero-length
    ///                                              messages (such messages nevertheless each
    ///                                              consume some system memory for bookkeeping
    ///                                              overhead).
    ///  * [`RLIMIT_NICE`] (since Linux 2.6.12) - Specifies a ceiling to which the process's nice
    ///                                           value can be raised using [`setpriority`] or
    ///                                           [`nice`]. The actual ceiling for the nice value
    ///                                           is calculated as `20 - rlim_cur`. (This
    ///                                           strangeness occurs because negative numbers
    ///                                           cannot be specified as resource limit values,
    ///                                           since they typically have special meanings. For
    ///                                           example, [`RLIM_INFINITY`] typically is the same
    ///                                           as -1.)
    ///  * [`RLIMIT_NOFILE`] - Specifies a value one greater than the maximum file descriptor
    ///                        number that can be opened by this process. Attempts ([`open`],
    ///                        [`pipe`], [`dup`], etc.) to exceed this limit yield the error
    ///                        [`EMFILE`]. (Historically, this limit was named [`RLIMIT_OFILE`] on
    ///                        BSD.)
    ///  * [`RLIMIT_NPROC`] - The maximum number of processes (or, more precisely on Linux,
    ///                       threads) that can be created for the real user ID of the calling
    ///                       process. Upon encountering this limit, [`fork`] fails with the error
    ///                       [`EAGAIN`].
    ///  * [`RLIMIT_RSS`] - Specifies the limit (in pages) of the process's resident set (the
    ///                     number of virtual pages resident in RAM). This limit only has effect in
    ///                     Linux 2.4.x, x < 30, and there only affects calls to [`madvise`]
    ///                     specifying [`MADV_WILLNEED`].
    ///  * [`RLIMIT_RTPRIO`] (Since Linux 2.6.12) - Specifies a ceiling on the real-time priority
    ///                                             that may be set for this process using
    ///                                             [`sched_setscheduler`] and [`sched_setparam`].
    ///  * [`RLIMIT_RTTIME`] (Since Linux 2.6.25) - Specifies a limit (in microseconds) on the
    ///                                             amount of CPU time that a process scheduled
    ///                                             under a real-time scheduling policy may consume
    ///                                             without making a blocking system call. For the
    ///                                             purpose of this limit, each time a process
    ///                                             makes a blocking system call, the count of its
    ///                                             consumed CPU time is reset to zero. The CPU
    ///                                             time count is not reset if the process
    ///                                             continues trying to use the CPU but is
    ///                                             preempted, its time slice expires, or it calls
    ///                                             [`sched_yield`]. Upon reaching the soft limit,
    ///                                             the process is sent a [`SIGXCPU`] signal. If
    ///                                             the process catches or ignores this signal and
    ///                                             continues consuming CPU time, then [`SIGXCPU`]
    ///                                             will be generated once each second until the
    ///                                             hard limit is reached, at which point the
    ///                                             process is sent a [`SIGKILL`] signal. The
    ///                                             intended use of this limit is to stop a runaway
    ///                                             real-time process from locking up the system.
    ///  * [`RLIMIT_SIGPENDING`] (Since Linux 2.6.8) - Specifies the limit on the number of signals
    ///                                                that may be queued for the real user ID of
    ///                                                the calling process. Both standard and
    ///                                                real-time signals are counted for the
    ///                                                purpose of checking this limit. However, the
    ///                                                limit is only enforced for [`sigqueue`]; it
    ///                                                is always possible to use [`kill`] to queue
    ///                                                one instance of any of the signals that are
    ///                                                not already queued to the process.
    ///  * [`RLIMIT_STACK`] - The maximum size of the process stack, in bytes. Upon reaching this
    ///                       limit, a [`SIGSEGV`] signal is generated. To handle this signal, a
    ///                       process must employ an alternate signal stack ([`sigaltstack`]).
    ///                       Since Linux 2.6.23, this limit also determines the amount of space
    ///                       used for the process's command-line arguments and environment
    ///                       variables; for details, see [`execve`].
    ///
    /// # Return Value
    /// On success, this system call returns 0. On error, -1 is returned, and [`errno`] is set
    /// appropriately.
    ///
    /// # Errors
    ///  * [`EFAULT`] - A pointer argument points to a location outside the accessible address
    ///                 space.
    ///  * [`EINVAL`] - The value specified in resource is not valid; or, for [`setrlimit`]:
    ///                 `rlim.rlim_cur` was greater than `rlim.rlim_max`.
    ///  * [`EPERM`] - An unprivileged process tried to raise the hard limit; the
    ///                [`CAP_SYS_RESOURCE`] capability is required to do this. Or, the caller tried
    ///                to increase the hard [`RLIMIT_NOFILE`] limit above the current kernel
    ///                maximum ([`NR_OPEN`]). Or, the calling process did not have permission to
    ///                set limits for the process specified by pid.
    ///  * [`ESRCH`] - Could not find a process with the ID specified in pid.
    pub fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
}
