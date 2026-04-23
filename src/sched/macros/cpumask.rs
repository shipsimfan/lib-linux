#[allow(missing_docs)]
#[macro_export]
macro_rules! __CPUMASK {
    ($cpu: expr) => {
        1 << ($cpu % $crate::sched::__NCPUBITS)
    };
}
