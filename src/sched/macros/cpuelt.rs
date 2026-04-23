#[allow(missing_docs)]
#[macro_export]
macro_rules! __CPUELT {
    ($cpu: expr) => {
        $cpu / $crate::sched::__NCPUBITS
    };
}
