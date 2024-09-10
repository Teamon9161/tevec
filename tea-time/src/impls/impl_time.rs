use std::fmt;
use std::str::FromStr;

use chrono::Timelike;
use tea_error::*;

use crate::Time;

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Time {
    type Err = TError;
    #[inline]
    fn from_str(s: &str) -> TResult<Self> {
        Self::parse(s, None)
    }
}

impl From<i64> for Time {
    #[inline]
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl Timelike for Time {
    #[inline]
    fn hour(&self) -> u32 {
        self.as_cr().unwrap().hour()
    }

    #[inline]
    fn minute(&self) -> u32 {
        self.as_cr().unwrap().minute()
    }

    #[inline]
    fn second(&self) -> u32 {
        self.as_cr().unwrap().second()
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        self.as_cr().unwrap().nanosecond()
    }

    #[inline]
    fn with_hour(&self, hour: u32) -> Option<Self> {
        if let Some(time) = self.as_cr() {
            if let Some(time) = time.with_hour(hour) {
                return Some(Self::from_cr(&time));
            }
        }
        None
    }

    #[inline]
    fn with_minute(&self, min: u32) -> Option<Self> {
        if let Some(time) = self.as_cr() {
            if let Some(time) = time.with_minute(min) {
                return Some(Self::from_cr(&time));
            }
        }
        None
    }

    #[inline]
    fn with_second(&self, sec: u32) -> Option<Self> {
        if let Some(time) = self.as_cr() {
            if let Some(time) = time.with_second(sec) {
                return Some(Self::from_cr(&time));
            }
        }
        None
    }

    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<Self> {
        if let Some(time) = self.as_cr() {
            if let Some(time) = time.with_nanosecond(nano) {
                return Some(Self::from_cr(&time));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_getters() {
        let time = Time::from_hms(12, 34, 56);
        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 34);
        assert_eq!(time.second(), 56);
        assert_eq!(time.nanosecond(), 0);
    }

    #[test]
    fn test_time_with_methods() {
        let time = Time::from_hms(12, 34, 56);

        let new_time = time.with_hour(15).unwrap();
        assert_eq!(new_time.hour(), 15);
        assert_eq!(new_time.minute(), 34);
        assert_eq!(new_time.second(), 56);

        let new_time = time.with_minute(45).unwrap();
        assert_eq!(new_time.hour(), 12);
        assert_eq!(new_time.minute(), 45);
        assert_eq!(new_time.second(), 56);

        let new_time = time.with_second(30).unwrap();
        assert_eq!(new_time.hour(), 12);
        assert_eq!(new_time.minute(), 34);
        assert_eq!(new_time.second(), 30);

        let new_time = time.with_nanosecond(500_000_000).unwrap();
        assert_eq!(new_time.hour(), 12);
        assert_eq!(new_time.minute(), 34);
        assert_eq!(new_time.second(), 56);
        assert_eq!(new_time.nanosecond(), 500_000_000);
    }

    #[test]
    fn test_invalid_time_modifications() {
        let time = Time::from_hms(12, 34, 56);

        assert!(time.with_hour(24).is_none());
        assert!(time.with_minute(60).is_none());
        assert!(time.with_second(60).is_none());
        assert!(time.with_nanosecond(3_000_000_000).is_none());
    }
}
