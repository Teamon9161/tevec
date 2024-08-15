use std::hash::Hash;
use std::str::FromStr;

use chrono::Duration;
use tea_error::{tbail, tensure, TError, TResult};

use crate::convert::*;

#[cfg(feature = "serde")]
#[serde_with::serde_as]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeDelta {
    pub months: i32,
    // #[cfg_attr(feature = "serde", serde_as(as = "serde_with::DurationSeconds<i64>"))]
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub inner: Duration,
}

#[cfg(not(feature = "serde"))]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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
    /// 1ns // 1 nanosecond
    /// 1us // 1 microsecond
    /// 1ms // 1 millisecond
    /// 1s  // 1 second
    /// 1m  // 1 minute
    /// 1h  // 1 hour
    /// 1d  // 1 day
    /// 1w  // 1 week
    /// 1mo // 1 calendar month
    /// 1y  // 1 calendar year
    ///
    /// Parse timedelta from string
    ///
    /// for example: "2y1mo-3d5h-2m3s"
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
