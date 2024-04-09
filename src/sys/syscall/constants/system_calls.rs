use std::ffi::c_long;

/// The `futex` system call provides a method for waiting until a certain condition becomes true.
/// It is typically used as a blocking construct in the context of shared-memory synchronization.
/// When using futexes, the majority of the synchronization operations are performed in user space.
/// A user- space program employs the `futex` system call only when it is likely that the program
/// has to block for a longer time until the condition becomes true. Other `futex` operations can
/// be used to wake any processes or threads waiting for a particular condition.
#[allow(non_upper_case_globals)]
pub const SYS_futex: c_long = 202;
