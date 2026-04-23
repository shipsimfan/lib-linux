/// Clears `set`, so that it contains no CPUs.
#[macro_export]
macro_rules! CPU_ZERO {
    ($set: expr) => {{
        let i_max = std::mem::size_of::<$crate::sched::cpu_set_t>()
            / std::mem::size_of::<$crate::sched::__cpu_mask>();
        let mut i = 0;
        while i < i_max {
            $set.__bits[i] = 0;
            i += 1;
        }
    }};
}
