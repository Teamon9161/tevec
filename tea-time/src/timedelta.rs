use std::hash::Hash;
use std::str::FromStr;

use chrono::Duration;
use tea_error::{tbail, tensure, TError, TResult};

use crate::convert::*;

#[cfg(feature = "serde")]
#[serde_with::serde_as]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
/// Represents a duration of time with both months and a more precise duration.
///
/// This struct combines a number of months with a `chrono::Duration` to represent
/// time intervals that may include calendar-specific units (months) as well as
/// fixed-length durations.
///
/// # Fields
///
/// * `months`: The number of months in the time delta.
/// * `inner`: A `chrono::Duration` representing the precise duration beyond whole months.
///
/// # Serialization
///
/// When the "serde" feature is enabled, this struct can be serialized and deserialized.
pub struct TimeDelta {
    pub months: i32,
    // #[cfg_attr(feature = "serde", serde_as(as = "serde_with::DurationSeconds<i64>"))]
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub inner: Duration,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
/// Represents a duration of time with both months and a more precise duration.
///
/// This struct combines a number of months with a `chrono::Duration` to represent
/// time intervals that may include calendar-specific units (months) as well as
/// fixed-length durations.
///
/// # Fields
///
/// * `months`: The number of months in the time delta.
/// * `inner`: A `chrono::Duration` representing the precise duration beyond whole months.
///
/// # Serialization
///
/// When the "serde" feature is enabled, this struct can be serialized and deserialized.
pub struct TimeDelta {
    pub months: i32,
    pub inner: Duration,
}

impl FromStr for TimeDelta {
    type Err = TError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TimeDelta::parse(s)
    }
}

impl From<&str> for TimeDelta {
    #[inline]
    fn from(s: &str) -> Self {
        TimeDelta::parse(s).unwrap_or_else(|e| panic!("{}", e))
    }
}

impl TimeDelta {
    /// Parse timedelta from string
    ///
    /// for example: "2y1mo-3d5h-2m3s"
    /// Parses a string representation of a duration into a `TimeDelta`.
    ///
    /// This function supports a variety of time units and can handle complex duration strings.
    ///
    /// # Supported Units
    ///
    /// - `ns`: nanoseconds
    ///
    /// - `us`: microseconds
    ///
    /// - `ms`: milliseconds
    ///
    /// - `s`: seconds
    ///
    /// - `m`: minutes
    ///
    /// - `h`: hours
    ///
    /// - `d`: days
    ///
    /// - `w`: weeks
    ///
    /// - `mo`: months
    ///
    /// - `y`: years
    ///
    /// # Format
    /// The duration string should be in the format of `<number><unit>`, and multiple such pairs can be combined.
    /// For example: "2y1mo-3d5h-2m3s" represents 2 years, 1 month, minus 3 days, 5 hours, minus 2 minutes, and 3 seconds.
    ///
    /// # Arguments
    /// * `duration` - A string slice that holds the duration to be parsed.
    ///
    /// # Returns
    /// * `TResult<Self>` - A Result containing the parsed `TimeDelta` if successful, or an error if parsing fails.
    ///
    /// # Examples
    /// ```
    /// use tea_time::TimeDelta;
    ///
    /// let td = TimeDelta::parse("1y2mo3d4h5m6s").unwrap();
    /// assert_eq!(td.months, 14); // 1 year and 2 months
    /// assert_eq!(td.inner, chrono::Duration::seconds(3 * 86400 + 4 * 3600 + 5 * 60 + 6));
    /// ```
    pub fn parse(duration: &str) -> TResult<Self> {
        let mut nsecs = 0;
        let mut secs = 0;
        let mut months = 0;
        let mut iter = duration.char_indices();
        let mut start = 0;
        let mut unit = String::with_capacity(2);
        while let Some((i, mut ch)) = iter.next() {
            if !ch.is_ascii_digit() && i != 0 {
                let n = duration[start..i].parse::<i64>().unwrap();
                loop {
                    if ch.is_ascii_alphabetic() {
                        unit.push(ch)
                    } else {
                        break;
                    }
                    match iter.next() {
                        Some((i, ch_)) => {
                            ch = ch_;
                            start = i
                        },
                        None => {
                            break;
                        },
                    }
                }
                tensure!(!unit.is_empty(), ParseError:"expected a unit in the duration string");

                match unit.as_str() {
                    "ns" => nsecs += n,
                    "us" => nsecs += n * NANOS_PER_MICRO,
                    "ms" => nsecs += n * NANOS_PER_MILLI,
                    "s" => secs += n,
                    "m" => secs += n * SECS_PER_MINUTE,
                    "h" => secs += n * SECS_PER_HOUR,
                    "d" => secs += n * SECS_PER_DAY,
                    "w" => secs += n * SECS_PER_WEEK,
                    "mo" => months += n as i32,
                    "y" => months += n as i32 * 12,
                    unit => tbail!(ParseError:"unit: '{}' not supported", unit),
                }
                unit.clear();
            }
        }
        let duration = Duration::seconds(secs) + Duration::nanoseconds(nsecs);
        Ok(TimeDelta {
            months,
            inner: duration,
        })
    }

    #[inline(always)]
    pub fn nat() -> Self {
        Self {
            months: i32::MIN,
            inner: Duration::seconds(0),
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_nat(&self) -> bool {
        self.months == i32::MIN
    }

    #[inline(always)]
    pub fn is_not_nat(&self) -> bool {
        self.months != i32::MIN
    }
}
