use crate::signal::sigaction_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EFAULT, EINVAL},
    signal::{
        SA_NOCLDSTOP, SA_NOCLDWAIT, SA_NODEFER, SA_NOMASK, SA_ONESHOT, SA_ONSTACK, SA_RESETHAND,
        SA_RESTART, SA_SIGINFO, SIGCHLD, SIGCONT, SIGKILL, SIGSTOP, SIGTSTP, SIGTTIN, SIGTTOU,
        SIG_DFL, SIG_IGN,
    },
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

extern "C" {
    /// The [`sigaction`] system call is used to change the action taken by a process on receipt of
    /// a specific signal.
    ///
    /// `signum` specifies the signal and can be any valid signal except [`SIGKILL`] and
    /// [`SIGSTOP`].
    ///
    /// If act is not [`null`], the new action for signal `signum` is installed from `act`. If
    /// `oldact` is not [`null_mut`], the previous action is saved in `oldact`.
    ///
    /// On some architectures a union is involved: do not assign to both
    /// `sigaction_t.handler.handler` and `sigaction_t.handler.sigaction`. The
    /// `sigaction_t.restorer` field is not intended for application use. `sigaction_t.sa_handler`
    /// specifies the action to be associated with `signum` and can be one of the following:
    ///  * [`SIG_DFL`] for the default action
    ///  * [`SIG_IGN`] to ignore this signal
    ///  * A pointer to a signal handling function. This function receives the signal number as its
    ///    only argument.
    ///
    /// If [`SA_SIGINFO`] is specified in `sigaction_t.flags`, then `sigaction_t.handler.sigaction`
    /// (instead of `sigaction_t.handler.handler`) specifies the signal-handling function for
    /// `signum`. This function receives three arguments, as described below.
    ///
    /// `sigaction_t.mask` specifies a mask of signals which should be blocked (i.e., added to the
    /// signal mask of the thread in which the signal handler is invoked) during execution of the
    /// signal handler. In addition, the signal which triggered the handler will be blocked, unless
    /// the [`SA_NODEFER`] flag is used.
    ///
    /// `sigaction_t.flags` specifies a set of flags which modify the behavior of the signal. It is
    /// formed by the bitwise OR of zero or more of the following:    
    ///  * [`SA_NOCLDSTOP`] - If `signum` is [`SIGCHLD`], do not receive notification when child
    ///                       processes stop (i.e., when they receive one of [`SIGSTOP`],
    ///                       [`SIGTSTP`], [`SIGTTIN`], or [`SIGTTOU`]) or resume (i.e., they
    ///                       receive [`SIGCONT`]). This flag is meaningful only when establishing
    ///                       a handler for [`SIGCHLD`].
    ///  * [`SA_NOCLDWAIT`] - If `signum` is [`SIGCHLD`], do not transform children into zombies
    ///                       when they terminate. This flag is meaningful only when establishing a
    ///                       handler for [`SIGCHLD`], or when setting that signal's disposition to
    ///                       [`SIG_DFL`]. If the [`SA_NOCLDWAIT`] flag is set when establishing a
    ///                       handler for [`SIGCHLD`], POSIX.1 leaves it unspecified whether a
    ///                       [`SIGCHLD`] signal is generated when a child process terminates. On
    ///                       Linux, a [`SIGCHLD`] signal is generated in this case; on some other
    ///                       implementations, it is not.
    ///  * [`SA_NODEFER`] - Do not add the signal to the thread's signal mask while the handler is
    ///                     executing, unless the signal is specified in `act.mask`. Consequently,
    ///                     a further instance of the signal may be delivered to the thread while
    ///                     it is executing the handler. This flag is meaningful only when
    ///                     establishing a signal handler. [`SA_NOMASK`] is an obsolete,
    ///                     nonstandard synonym for this flag.
    ///  * [`SA_ONSTACK`] - Call the signal handler on an alternate signal stack provided by
    ///                     [`sigaltstack`]. If an alternate stack is not available, the default
    ///                     stack will be used. This flag is meaningful only when establishing a
    ///                     signal handler.
    ///  * [`SA_RESETHAND`] - Restore the signal action to the default upon entry to the signal
    ///                       handler. This flag is meaningful only when establishing a signal
    ///                       handler. [`SA_ONESHOT`] is an obsolete, nonstandard synonym for this
    ///                       flag.
    ///  * [`SA_RESTART`] - Provide behavior compatible with BSD signal semantics by making certain
    ///                     system calls restartable across signals. This flag is meaningful only
    ///                     when establishing a signal handler.
    ///  * [`SA_SIGINFO`] - The signal handler takes three arguments, not one. In this case,
    ///                     `sigaction_t.handler.sigaction` should be set instead of
    ///                     `sigaction_t.handler.handler`. This flag is meaningful only when
    ///                     establishing a signal handler.
    /// # Return Value
    /// [`sigaction`] returns 0 on success; on error, -1 is returned, and [`errno`] is set to
    /// indicate the error.
    ///
    /// # Errors
    ///  * [`EFAULT`] - `act` or `oldact` points to memory which is not a valid part of the process
    ///                 address space.
    ///  * [`EINVAL`] - An invalid signal was specified. This will also be generated if an attempt
    ///                 is made to change the action for [`SIGKILL`] or [`SIGSTOP`], which cannot
    ///                 be caught or ignored.
    pub fn sigaction(signum: c_int, act: *const sigaction_t, oldact: *mut sigaction_t) -> c_int;
}
