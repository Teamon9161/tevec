use super::timeunit::*;
use crate::DateTime;

/// The number of nanoseconds in a microsecond.
pub const NANOS_PER_MICRO: i64 = 1000;
/// The number of nanoseconds in a millisecond.
pub const NANOS_PER_MILLI: i64 = 1_000_000;
/// The number of nanoseconds in seconds.
pub const NANOS_PER_SEC: i64 = 1_000_000_000;
/// The number of microseconds per millisecond.
pub const MICROS_PER_MILLI: i64 = 1000;
/// The number of microseconds per second.
pub const MICROS_PER_SEC: i64 = 1_000_000;
/// The number of milliseconds per second.
pub const MILLIS_PER_SEC: i64 = 1000;
/// The number of seconds in a minute.
pub const SECS_PER_MINUTE: i64 = 60;
/// The number of seconds in an hour.
pub const SECS_PER_HOUR: i64 = 3600;
/// The number of (non-leap) seconds in days.
pub const SECS_PER_DAY: i64 = 86400;
/// The number of (non-leap) seconds in a week.
pub const SECS_PER_WEEK: i64 = 604800;

impl<U: TimeUnitTrait> DateTime<U> {
    pub fn into_unit<T: TimeUnitTrait>(self) -> DateTime<T> {
        if U::unit() == T::unit() {
            unsafe { std::mem::transmute(self) }
        } else {
            use TimeUnit::*;
            match (U::unit(), T::unit()) {
                (Nanosecond, Microsecond) => DateTime::new(self.0 / NANOS_PER_MICRO),
                (Nanosecond, Millisecond) => DateTime::new(self.0 / NANOS_PER_MILLI),
                (Nanosecond, Second) => DateTime::new(self.0 / NANOS_PER_SEC),
                (Microsecond, Millisecond) => DateTime::new(self.0 / MICROS_PER_MILLI),
                (Microsecond, Second) => DateTime::new(self.0 / MICROS_PER_SEC),
                (Millisecond, Second) => DateTime::new(self.0 / MILLIS_PER_SEC),
                (Microsecond, Nanosecond) => DateTime::new(self.0 * NANOS_PER_MICRO),
                (Millisecond, Nanosecond) => DateTime::new(self.0 * NANOS_PER_MILLI),
                (Second, Nanosecond) => DateTime::new(self.0 * NANOS_PER_SEC),
                (Millisecond, Microsecond) => DateTime::new(self.0 * MICROS_PER_MILLI),
                (Second, Microsecond) => DateTime::new(self.0 * MICROS_PER_SEC),
                (Second, Millisecond) => DateTime::new(self.0 * MILLIS_PER_SEC),
                // currently unit should only in [Nanosecond, Microsecond, Millisecond, Second]
                (u1, u2) => unimplemented!("convert from {:?} to {:?} is not implemented", u1, u2),
            }
        }
    }
}
