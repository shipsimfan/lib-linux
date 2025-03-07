/// Convert a linux system call result (-1 on error) into a [`Result<c_int>`]
#[macro_export]
macro_rules! try_linux {
    ($expr: expr) => {
        match unsafe { $expr } {
            -1 => Err($crate::Error::errno()),
            result => Ok(result),
        }
    };
}
