/// Test to see if CPU `cpu` is a member of `set`.
#[macro_export]
macro_rules! CPU_ISSET {
    ($cpu: expr, $set: expr) => {{
        let cpu = $cpu as usize;
        if cpu / 8 < std::mem::size_of::<$crate::sched::cpu_set_t>() {
            ($set.__bits[$crate::__CPUELT!(cpu)] & $crate::__CPUMASK!(cpu)) != 0
        } else {
            false
        }
    }};
}
