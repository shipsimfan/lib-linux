/// Return the number of CPUs in `set`.
#[macro_export]
macro_rules! CPU_COUNT {
    ($set: expr) => {
        $crate::sched::__sched_cpucount(std::mem::size_of::<$crate::sched::cpu_set_t>(), $set)
    };
}
