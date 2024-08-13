// rustdoc imports
#[allow(unused_imports)]
use crate::time::{daylight, timezone, tzname};

extern "C" {
    /// The [`tzset`] function initializes the [`tzname`] variable from the `TZ` environment
    /// variable. This function is automatically called by the other time conversion functions that
    /// depend on the timezone. In a System-V-like environment, it will also set the variables
    /// [`timezone`] (seconds West of UTC) and [`daylight`] (to 0 if this timezone does not have
    /// any daylight saving time rules, or to nonzero if there is a time during the year when
    /// daylight saving time applies).
    pub fn tzset();
}
