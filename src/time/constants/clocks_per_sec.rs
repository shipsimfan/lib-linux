use crate::time::clock_t;

/// The macro [`CLOCKS_PER_SEC`] is an expression with type [`clock_t`] that is the number per
/// second of the value returned by the `clock` function.
pub const CLOCKS_PER_SEC: clock_t = 1000000;
