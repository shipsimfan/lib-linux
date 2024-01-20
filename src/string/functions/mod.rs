mod strerror;
mod strerror_l;

#[cfg(not(target_env = "gnu"))]
mod strerror_r;
#[cfg(target_env = "gnu")]
mod strerror_r_gnu;

pub use strerror::strerror;
pub use strerror_l::strerror_l;

#[cfg(not(target_env = "gnu"))]
pub use strerror_r::strerror_r;
#[cfg(target_env = "gnu")]
pub use strerror_r_gnu::strerror_r;
