use std::ffi::c_int;

/// Identifier for system-wide realtime clock.
pub const CLOCK_REALTIME: c_int = 0;

/// Monotonic system-wide clock.
pub const CLOCK_MONOTONIC: c_int = 1;

/// High-resolution timer from the CPU.
pub const CLOCK_PROCESS_CPUTIME_ID: c_int = 2;

/// Thread-specific CPU-time clock.
pub const CLOCK_THREAD_CPUTIME_ID: c_int = 3;

/// Monotonic system-wide clock, not adjusted for frequency scaling.
pub const CLOCK_MONOTONIC_RAW: c_int = 4;

/// Identifier for system-wide realtime clock, updated only on ticks.
pub const CLOCK_REALTIME_COARSE: c_int = 5;

/// Monotonic system-wide clock, updated only on ticks.
pub const CLOCK_MONOTONIC_COARSE: c_int = 6;

/// Monotonic system-wide clock that includes time spent in suspension.
pub const CLOCK_BOOTTIME: c_int = 7;

/// Like [`CLOCK_REALTIME`] but also wakes suspended system.
pub const CLOCK_REALTIME_ALARM: c_int = 8;

/// Like [`CLOCK_BOOTTIME`] but also wakes suspended system.
pub const CLOCK_BOOTTIME_ALARM: c_int = 9;

/// Like [`CLOCK_REALTIME`] but in International Atomic Time.
pub const CLOCK_TAI: c_int = 11;
