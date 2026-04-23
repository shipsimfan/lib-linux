use crate::sched::{__NCPUBITS, __cpu_mask, CPU_SETSIZE};

/// Data structure to describe CPU mask.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct cpu_set_t {
    #[allow(missing_docs)]
    pub __bits: [__cpu_mask; CPU_SETSIZE / __NCPUBITS],
}

impl Default for cpu_set_t {
    fn default() -> Self {
        cpu_set_t {
            __bits: [0; CPU_SETSIZE / __NCPUBITS],
        }
    }
}
