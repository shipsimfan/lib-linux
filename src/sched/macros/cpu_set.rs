/// Add CPU `cpu` to `set`.
#[macro_export]
macro_rules! CPU_SET {
    ($cpu: expr, $set: expr) => {{
        let cpu = $cpu as usize;
        if cpu / 8 < std::mem::size_of::<$crate::sched::cpu_set_t>() {
            $set.__bits[$crate::__CPUELT!(cpu)] |= $crate::__CPUMASK!(cpu);
        }
    }};
}
