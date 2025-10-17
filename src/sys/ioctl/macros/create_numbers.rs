#[allow(missing_docs)]
#[macro_export]
macro_rules! _IOC {
    ($dir: expr, $type: expr, $nr: expr, $size: expr) => {
        ($dir as std::ffi::c_ulong) << $crate::sys::ioctl::_IOC_DIRSHIFT
            | ($type as std::ffi::c_ulong) << $crate::sys::ioctl::_IOC_TYPESHIFT
            | ($nr as std::ffi::c_ulong) << $crate::sys::ioctl::_IOC_NRSHIFT
            | ($size as std::ffi::c_ulong) << $crate::sys::ioctl::_IOC_SIZESHIFT
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! _IOC_TYPECHECK {
    ($t: ty) => {
        std::mem::size_of::<$t>() as std::ffi::c_ulong
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! _IO {
    ($type: expr, $nr: expr) => {
        $crate::_IOC!($crate::sys::ioctl::_IOC_NONE, $type, $nr, 0)
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! _IOR {
    ($type: expr, $nr: expr, $size: ty) => {
        $crate::_IOC!(
            $crate::sys::ioctl::_IOC_READ,
            $type,
            $nr,
            $crate::_IOC_TYPECHECK!($size)
        )
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! _IOW {
    ($type: expr, $nr: expr, $size: ty) => {
        $crate::_IOC!(
            $crate::sys::ioctl::_IOC_WRITE,
            $type,
            $nr,
            $crate::_IOC_TYPECHECK!($size)
        )
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! _IOWR {
    ($type: expr, $nr: expr, $size: ty) => {
        $crate::_IOC!(
            $crate::sys::ioctl::_IOC_READ | $crate::sys::ioctl::_IOC_WRITE,
            $type,
            $nr,
            $crate::_IOC_TYPECHECK!($size)
        )
    };
}
