use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::EAGAIN,
    linux::futex::FUTEX_BITSET_MATCH_ANY,
    time::{CLOCK_MONOTONIC, CLOCK_REALTIME},
    unistd::close,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// This operation tests that the value at the futex `word` pointed to by the address `uaddr` still
/// contains the expected value `val`, and if so, then sleeps waiting for a [`FUTEX_WAKE`]
/// operation on the futex `word`. The load of the value of the futex `word` is an atomic memory
/// access (i.e., using atomic machine instructions of the respective architecture). This load, the
/// comparison with the expected value, and starting to sleep are performed atomically and totally
/// ordered with respect to other futex operations on the same futex `word`. If the thread starts
/// to sleep, it is considered a waiter on this futex `word`. If the futex value does not match
/// `val`, then the call fails immediately with the error [`EAGAIN`].
///
/// The purpose of the comparison with the expected value is to prevent lost wake-ups. If another
/// thread changed the value of the futex `word` after the calling thread decided to block based on
/// the prior value, and if the other thread executed a [`FUTEX_WAKE`] operation (or similar
/// wake-up) after the value change and before this [`FUTEX_WAIT`] operation, then the calling
/// thread will observe the value change and will not start to sleep.
///
/// If the `timeout` is not [`null_mut`], the structure it points to specifies a timeout for the
/// wait. (This interval will be rounded up to the system clock granularity, and is guaranteed not
/// to expire early.) The timeout is by default measured according to the [`CLOCK_MONOTONIC`]
/// clock, but, since Linux 4.5, the [`CLOCK_REALTIME`] clock can be selected by specifying
/// [`FUTEX_CLOCK_REALTIME`] in `futex_op`. If `timeout` is [`null_mut`], the call blocks
/// indefinitely.
///
/// Note: for [`FUTEX_WAIT`], `timeout` is interpreted as a relative value.  This differs from
/// other futex operations, where `timeout` is interpreted as an absolute value.  To obtain the
/// equivalent of [`FUTEX_WAIT`] with an absolute timeout, employ [`FUTEX_WAIT_BITSET`] with `val3`
/// specified as [`FUTEX_BITSET_MATCH_ANY`].
///
/// The arguments `uaddr2` and `val3` are ignored.
pub const FUTEX_WAIT: c_int = 0;

/// This operation wakes at most `val` of the waiters that are waiting (e.g., inside
/// [`FUTEX_WAIT`]) on the futex `word` at the address `uaddr`.  Most commonly, `val` is specified
/// as either 1 (wake up a single waiter) or [`c_int::MAX`] (wake up all waiters).  No guarantee is
/// provided about which waiters are awoken (e.g., a waiter with a higher scheduling priority is
/// not guaranteed to be awoken in preference to a waiter with a lower priority).
///
/// The arguments `timeout`, `uaddr2`, and `val3` are ignored.
pub const FUTEX_WAKE: c_int = 1;

/// This operation creates a file descriptor that is associated with the futex at `uaddr`. The
/// caller must [`close`] the returned file descriptor after use. When another process or thread
/// performs a [`FUTEX_WAKE`] on the futex `word`, the file descriptor indicates as being readable
/// with [`select`], [`poll`], and [`epoll`].
///
/// The file descriptor can be used to obtain asynchronous notifications: if `val` is nonzero,
/// then, when another process or thread executes a [`FUTEX_WAKE`], the caller will receive the
/// signal number that was passed in `val`.
///
/// The arguments `timeout`, `uaddr2`, and `val3` are ignored.
///
/// Because it was inherently racy, [`FUTEX_FD`] has been removed from Linux 2.6.26 onward.
pub const FUTEX_FD: c_int = 2;

/// This operation performs the same task as [`FUTEX_CMP_REQUEUE`], except that no check is made
/// using the value in `val3`. (The argument `val3` is ignored.)
pub const FUTEX_REQUEUE: c_int = 3;

/// This operation first checks whether the location `uaddr` still contains the value `val3`. If
/// not, the operation fails with the error [`EAGAIN`]. Otherwise, the operation wakes up a maximum
/// of `val` waiters that are waiting on the futex at `uaddr`. If there are more than `val`
/// waiters, then the remaining waiters are removed from the wait queue of the source futex at
/// `uaddr` and added to the wait queue of the target futex at `uaddr2`. The `val2` argument
/// specifies an upper limit on the number of waiters that are requeued to the futex at `uaddr2`.
///
/// The load from `uaddr` is an atomic memory access (i.e., using atomic machine instructions of
/// the respective architecture). This load, the comparison with `val3`, and the requeueing of any
/// waiters are performed atomically and totally ordered with respect to other operations on the
/// same futex word.
///
/// Typical values to specify for `val` are `0` or `1`. (Specifying [`c_int::MAX`] is not useful,
/// because it would make the [`FUTEX_CMP_REQUEUE`] operation equivalent to [`FUTEX_WAKE`].) The
/// limit value specified via `val2` is typically either 1 or [`INT_MAX`]. (Specifying the argument
/// as 0 is not useful, because it would make the [`FUTEX_CMP_REQUEUE`] operation equivalent to
/// [`FUTEX_WAIT`].)
///
/// The [`FUTEX_CMP_REQUEUE`] operation was added as a replacement for the earlier
/// [`FUTEX_REQUEUE`]. The difference is that the check of the value at uaddr can be used to ensure
/// that requeueing happens only under certain conditions, which allows race conditions to be
/// avoided in certain use cases.
///
/// Both [`FUTEX_REQUEUE`] and [`FUTEX_CMP_REQUEUE`] can be used to avoid "thundering herd"
/// wake-ups that could occur when using [`FUTEX_WAKE`] in cases where all of the waiters that are
/// woken need to acquire another futex. Consider the following scenario, where multiple waiter
/// threads are waiting on `b`, a wait queue implemented using a futex:
///
/// ```no_run
/// lock(a)     
/// while !check_value(v) {         
///     unlock(a);         
///     block_on(b);         
///     lock(a);     
/// }   
///
/// unlock(a);
/// ```
///
/// If a waker thread used [`FUTEX_WAKE`], then all waiters waiting on B would be woken up, and
/// they would all try to acquire lock `a`. However, waking all of the threads in this manner would
/// be pointless because all except one of the threads would immediately block on lock `a` again.
/// By contrast, a requeue operation wakes just one waiter and moves the other waiters to lock `a`,
/// and when the woken waiter unlocks `a` then the next waiter can proceed.
pub const FUTEX_CMP_REQUEUE: c_int = 4;

/// This option bit can be employed with all futex operations. It tells the kernel that the futex
/// is process-private and not shared with another process (i.e., it is being used for
/// synchronization only between threads of the same process). This allows the kernel to make some
/// additional performance optimizations.

pub const FUTEX_PRIVATE_FLAG: c_int = 128;

/// This option bit can be employed only with the [`FUTEX_WAIT_BITSET`], [`FUTEX_WAIT_REQUEUE_PI`],
/// (since Linux 4.5) [`FUTEX_WAIT`], and (since Linux 5.14) [`FUTEX_LOCK_PI2`] operations.
///
/// If this option is set, the kernel measures the timeout against the [`CLOCK_REALTIME`] clock.
///
/// If this option is not set, the kernel measures the timeout against the [`CLOCK_MONOTONIC`]
/// clock.
pub const FUTEX_CLOCK_REALTIME: c_int = 256;
