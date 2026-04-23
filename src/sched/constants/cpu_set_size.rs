use crate::sched::__cpu_mask;

/// Size definition for CPU sets.
pub const CPU_SETSIZE: usize = 1024;

#[allow(missing_docs)]
pub const __NCPUBITS: usize = 8 * std::mem::size_of::<__cpu_mask>();
