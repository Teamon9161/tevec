use chrono::{NaiveTime, Timelike};
use tea_error::{TError, TResult};

use crate::convert::*;
/// Represents a time of day with nanosecond precision.
///
/// This struct is a wrapper around an `i64` value representing the number of nanoseconds
/// since midnight. It provides various methods for creating, manipulating, and converting
/// time values.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time(pub i64);

impl Time {
    /// Creates a new `Time` instance from the given number of nanoseconds.
    #[inline]
    pub const fn from_i64(nanos: i64) -> Self {
        Self(nanos)
    }

    /// Checks if the time is "not a time" (NAT).
    ///
    /// Returns `true` if the internal value is `i64::MIN`, which represents an invalid time.
    #[inline]
    pub const fn is_nat(&self) -> bool {
        self.0 == i64::MIN
    }

    /// Checks if the time is a valid time (not NAT).
    ///
    /// Returns `true` if the internal value is not `i64::MIN`.
    #[inline]
    pub const fn is_not_nat(&self) -> bool {
        self.0 != i64::MIN
    }

    /// Returns a `Time` instance representing "not a time" (NAT).
    #[inline]
    pub const fn nat() -> Self {
        Self(i64::MIN)
    }

    /// Converts the `Time` instance to its raw `i64` value.
    #[inline]
    pub const fn into_i64(self) -> i64 {
        self.0
    }

    /// Creates a `Time` instance from a `chrono::NaiveTime`.
    #[inline]
    pub fn from_cr(cr: &NaiveTime) -> Self {
        Self(cr.num_seconds_from_midnight() as i64 * NANOS_PER_SEC + cr.nanosecond() as i64)
    }

    /// Converts the `Time` instance to a `chrono::NaiveTime`, if valid.
    #[inline]
    pub const fn as_cr(&self) -> Option<NaiveTime> {
        use NANOS_PER_SEC;
        let secs = self.0 / NANOS_PER_SEC;
        let nanos = self.0 % NANOS_PER_SEC;
        NaiveTime::from_num_seconds_from_midnight_opt(secs as u32, nanos as u32)
    }

    /// Creates a `Time` instance from hours, minutes, and seconds.
    #[inline]
    pub const fn from_hms(hour: i64, min: i64, sec: i64) -> Self {
        let secs = hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec;
        let nanos = secs * NANOS_PER_SEC;
        Self(nanos)
    }

    /// Creates a `Time` instance from hours, minutes, seconds, and milliseconds.
    #[inline]
    pub const fn from_hms_milli(hour: i64, min: i64, sec: i64, milli: i64) -> Self {
        let mut time = Self::from_hms(hour, min, sec);
        let nanos = milli * NANOS_PER_MILLI;
        time.0 += nanos;
        time
    }

    /// Creates a `Time` instance from hours, minutes, seconds, and microseconds.
    #[inline]
    pub const fn from_hms_micro(hour: i64, min: i64, sec: i64, micro: i64) -> Self {
        let mut time = Self::from_hms(hour, min, sec);
        let nanos = micro * NANOS_PER_MICRO;
        time.0 += nanos;
        time
    }

    /// Creates a `Time` instance from hours, minutes, seconds, and nanoseconds.
    #[inline]
    pub const fn from_hms_nano(hour: i64, min: i64, sec: i64, nano: i64) -> Self {
        let mut time = Self::from_hms(hour, min, sec);
        time.0 += nano;
        time
    }

    /// Creates a `Time` instance from the number of seconds since midnight and additional nanoseconds.
    #[inline]
    pub const fn from_num_seconds_from_midnight(secs: i64, nano: i64) -> Self {
        let nanos = secs * NANOS_PER_SEC + nano;
        Self(nanos)
    }

    /// Parses a string into a `Time` instance using an optional format string.
    ///
    /// If no format is provided, it attempts to parse using the default format.
    #[inline]
    pub fn parse(s: &str, fmt: Option<&str>) -> TResult<Self> {
        let naive_time = if let Some(fmt) = fmt {
            NaiveTime::parse_from_str(s, fmt)
        } else {
            s.parse()
        }
        .map_err(|e| TError::ParseError(e.to_string().into()))?;
        Ok(Time(
            naive_time.num_seconds_from_midnight() as i64 * NANOS_PER_SEC
                + naive_time.nanosecond() as i64,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hms() {
        let time = Time::from_hms(12, 34, 56);
        assert_eq!(time.0, (12 * 3600 + 34 * 60 + 56) * NANOS_PER_SEC);
    }

    #[test]
    fn test_from_hms_milli() {
        let time = Time::from_hms_milli(12, 34, 56, 789);
        assert_eq!(
            time.0,
            (12 * 3600 + 34 * 60 + 56) * NANOS_PER_SEC + 789 * NANOS_PER_MILLI
        );
    }

    #[test]
    fn test_from_hms_micro() {
        let time = Time::from_hms_micro(12, 34, 56, 789);
        assert_eq!(
            time.0,
            (12 * 3600 + 34 * 60 + 56) * NANOS_PER_SEC + 789 * NANOS_PER_MICRO
        );
    }

    #[test]
    fn test_from_hms_nano() {
        let time = Time::from_hms_nano(12, 34, 56, 789);
        assert_eq!(time.0, (12 * 3600 + 34 * 60 + 56) * NANOS_PER_SEC + 789);
    }

    #[test]
    fn test_from_num_seconds_from_midnight() {
        let time = Time::from_num_seconds_from_midnight(45296, 789);
        assert_eq!(time.0, 45296 * NANOS_PER_SEC + 789);
    }

    #[test]
    fn test_parse() {
        let time = Time::parse("12:34:56", None).unwrap();
        assert_eq!(time.0, (12 * 3600 + 34 * 60 + 56) * NANOS_PER_SEC);

        let time = Time::parse("12:34:56.789", Some("%H:%M:%S%.3f")).unwrap();
        assert_eq!(
            time.0,
            (12 * 3600 + 34 * 60 + 56) * NANOS_PER_SEC + 789 * NANOS_PER_MILLI
        );

        assert!(Time::parse("invalid", None).is_err());
    }
}
