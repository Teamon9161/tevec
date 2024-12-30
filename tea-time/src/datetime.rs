use std::cmp::Ordering;
use std::hash::Hash;
use std::marker::PhantomData;

use chrono::{
    DateTime as CrDateTime, Datelike, DurationRound, Months, NaiveDate, NaiveDateTime, NaiveTime,
    Timelike, Utc,
};
use tea_error::{tbail, TResult};

use super::timeunit::*;
use crate::TimeDelta;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
/// Represents a date and time with a specific time unit precision.
///
/// # Type Parameters
///
/// * `U`: The time unit precision, defaulting to `Nanosecond`. Must implement `TimeUnitTrait`.
///
/// # Fields
///
/// * `0`: An `i64` representing the timestamp in the specified time unit.
/// * `PhantomData<U>`: A zero-sized type used to "mark" the time unit without affecting the struct's memory layout.
pub struct DateTime<U: TimeUnitTrait = Nanosecond>(pub i64, PhantomData<U>);

impl<U: TimeUnitTrait> std::fmt::Debug for DateTime<U>
where
    Self: TryInto<CrDateTime<Utc>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nat() {
            write!(f, "NaT")
        } else {
            write!(f, "{}", self.strftime(None))
        }
    }
}

unsafe impl<U: TimeUnitTrait> Send for DateTime<U> {}
unsafe impl<U: TimeUnitTrait> Sync for DateTime<U> {}

const TIME_RULE_VEC: [&str; 11] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M:%S.%f",
    "%Y-%m-%d",
    "%Y%m%d",
    "%Y%m%d %H%M%S",
    "%d/%m/%Y",
    "%d/%m/%Y H%M%S",
    "%Y%m%d%H%M%S",
    "%d/%m/%YH%M%S",
    "%Y/%m/%d",
    "%Y/%m/%d %H:%M:%S",
];

impl<U: TimeUnitTrait> DateTime<U> {
    /// Creates a new `DateTime` instance with the given timestamp.
    ///
    /// # Arguments
    ///
    /// * `dt` - An `i64` representing the timestamp in the specified time unit.
    ///
    /// # Returns
    ///
    /// A new `DateTime<U>` instance.
    #[inline]
    pub const fn new(dt: i64) -> Self {
        Self(dt, PhantomData)
    }

    /// Checks if the `DateTime` instance represents "Not-a-Time" (NaT).
    ///
    /// # Returns
    ///
    /// `true` if the instance is NaT, `false` otherwise.
    #[inline]
    pub const fn is_nat(&self) -> bool {
        self.0 == i64::MIN
    }

    /// Checks if the `DateTime` instance represents a valid time (not NaT).
    ///
    /// # Returns
    ///
    /// `true` if the instance is not NaT, `false` otherwise.
    #[inline]
    pub const fn is_not_nat(&self) -> bool {
        self.0 != i64::MIN
    }

    /// Creates a new `DateTime` instance representing "Not-a-Time" (NaT).
    ///
    /// # Returns
    ///
    /// A new `DateTime<U>` instance representing NaT.
    #[inline]
    pub const fn nat() -> Self {
        Self(i64::MIN, PhantomData)
    }

    /// Converts the `DateTime` instance to its underlying `i64` timestamp.
    ///
    /// # Returns
    ///
    /// The `i64` timestamp value.
    #[inline]
    pub const fn into_i64(self) -> i64 {
        self.0
    }

    /// Creates a `DateTime` instance from an optional `i64` timestamp.
    ///
    /// # Arguments
    ///
    /// * `v` - An `Option<i64>` representing the timestamp.
    ///
    /// # Returns
    ///
    /// A new `DateTime<U>` instance. If `v` is `None`, returns NaT.
    #[inline]
    pub const fn from_opt_i64(v: Option<i64>) -> Self {
        if let Some(v) = v {
            Self::new(v)
        } else {
            Self::nat()
        }
    }

    /// Converts the `DateTime` instance to an optional `i64` timestamp.
    ///
    /// # Returns
    ///
    /// `Some(i64)` if the instance is not NaT, `None` otherwise.
    #[inline]
    pub const fn into_opt_i64(self) -> Option<i64> {
        if self.is_nat() {
            None
        } else {
            Some(self.0)
        }
    }

    /// Converts the `DateTime` instance to a `chrono::DateTime<Utc>`.
    ///
    /// # Returns
    ///
    /// `Some(CrDateTime<Utc>)` if the conversion is successful, `None` if the instance is NaT.
    #[inline]
    #[deprecated(since = "0.5.0", note = "use `as_cr` instead")]
    pub fn to_cr(&self) -> Option<CrDateTime<Utc>>
    where
        Self: TryInto<CrDateTime<Utc>>,
    {
        self.as_cr()
    }

    #[inline]
    pub fn as_cr(&self) -> Option<CrDateTime<Utc>>
    where
        Self: TryInto<CrDateTime<Utc>>,
    {
        if self.is_nat() {
            None
        } else {
            (*self).try_into().ok()
        }
    }

    /// Parses a string into a `DateTime` instance.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse.
    /// * `fmt` - An optional format string. If `None`, tries multiple common formats.
    ///
    /// # Returns
    ///
    /// A `TResult<Self>` containing the parsed `DateTime` or an error.
    #[inline(always)]
    pub fn parse(s: &str, fmt: Option<&str>) -> TResult<Self>
    where
        Self: From<CrDateTime<Utc>>,
    {
        if let Some(fmt) = fmt {
            if let Ok(cr_dt) = NaiveDateTime::parse_from_str(s, fmt) {
                Ok(cr_dt.into())
            } else if let Ok(cr_date) = NaiveDate::parse_from_str(s, fmt) {
                Ok(cr_date.into())
            } else {
                tbail!(ParseError:"Failed to parse datetime from string: {}", s)
            }
        } else {
            for fmt in TIME_RULE_VEC.iter() {
                if let Ok(cr_dt) = NaiveDateTime::parse_from_str(s, fmt) {
                    return Ok(cr_dt.into());
                } else if let Ok(cr_date) = NaiveDate::parse_from_str(s, fmt) {
                    return Ok(cr_date.into());
                }
            }
            tbail!(ParseError:"Failed to parse datetime from string: {}", s)
        }
    }

    /// Formats the `DateTime` instance as a string.
    ///
    /// # Arguments
    ///
    /// * `fmt` - An optional format string. If `None`, uses "%Y-%m-%d %H:%M:%S.%f".
    ///
    /// # Returns
    ///
    /// A formatted string representation of the `DateTime`.
    #[inline]
    pub fn strftime(&self, fmt: Option<&str>) -> String
    where
        Self: TryInto<CrDateTime<Utc>>,
    {
        if self.is_nat() {
            "NaT".to_string()
        } else {
            let fmt = fmt.unwrap_or("%Y-%m-%d %H:%M:%S.%f");
            self.as_cr().unwrap().format(fmt).to_string()
        }
    }

    /// Truncates the `DateTime` to a specified duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - A `TimeDelta` specifying the truncation interval.
    ///
    /// # Returns
    ///
    /// A new `DateTime<U>` instance truncated to the specified duration.
    pub fn duration_trunc(self, duration: TimeDelta) -> Self
    where
        Self: TryInto<CrDateTime<Utc>> + From<CrDateTime<Utc>>,
    {
        if self.is_nat() {
            return self;
        }
        let mut dt = self.as_cr().unwrap();
        let dm = duration.months;
        if dm != 0 {
            let (flag, dt_year) = dt.year_ce();
            if dm < 0 {
                unimplemented!("not support year before ce or negative month")
            }
            let dt_month = if flag {
                (dt_year * 12 + dt.month()) as i32
            } else {
                dt_year as i32 * (-12) + dt.month() as i32
            };
            let delta_down = dt_month % dm;
            dt = match delta_down.cmp(&0) {
                Ordering::Equal => dt,
                Ordering::Greater => dt - Months::new(delta_down as u32),
                Ordering::Less => dt - Months::new((dm - delta_down.abs()) as u32),
            };
            if let Some(nd) = duration.inner.num_nanoseconds() {
                if nd == 0 {
                    return dt.into();
                }
            }
        }
        dt.duration_trunc(duration.inner)
            .expect("Rounding Error")
            .into()
    }
}

impl<U: TimeUnitTrait> DateTime<U>
where
    Self: TryInto<CrDateTime<Utc>>,
{
    /// Returns the time component of the DateTime as a NaiveTime.
    ///
    /// # Returns
    ///
    /// `Option<NaiveTime>`: The time component if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn time(&self) -> Option<NaiveTime> {
        self.as_cr().map(|dt| dt.time())
    }

    /// Returns the year.
    ///
    /// # Returns
    ///
    /// `Option<i32>`: The year if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn year(&self) -> Option<i32> {
        self.as_cr().map(|dt| dt.year())
    }

    /// Returns the day of the month (1-31).
    ///
    /// # Returns
    ///
    /// `Option<usize>`: The day of the month if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn day(&self) -> Option<usize> {
        self.as_cr().map(|dt| dt.day() as usize)
    }

    /// Returns the month (1-12).
    ///
    /// # Returns
    ///
    /// `Option<usize>`: The month if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn month(&self) -> Option<usize> {
        self.as_cr().map(|dt| dt.month() as usize)
    }

    /// Returns the hour (0-23).
    ///
    /// # Returns
    ///
    /// `Option<usize>`: The hour if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn hour(&self) -> Option<usize> {
        self.as_cr().map(|dt| dt.hour() as usize)
    }

    /// Returns the minute (0-59).
    ///
    /// # Returns
    ///
    /// `Option<usize>`: The minute if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn minute(&self) -> Option<usize> {
        self.as_cr().map(|dt| dt.minute() as usize)
    }

    /// Returns the second (0-59).
    ///
    /// # Returns
    ///
    /// `Option<usize>`: The second if the DateTime is valid, or None if it's NaT.
    #[inline(always)]
    pub fn second(&self) -> Option<usize> {
        self.as_cr().map(|dt| dt.second() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_assignments, unused_variables)]
    fn test_parse_datetime() -> TResult<()> {
        let mut dt: DateTime = "2020-01-01 00:00:00".parse()?;
        dt = DateTime::parse("2020-01-01", Some("%Y-%m-%d"))?;
        dt = "2020-01-01".parse()?;
        dt = "20220101".parse()?;
        dt = "2021/02/03".parse()?;
        Ok(())
    }

    #[test]
    fn test_datetime_components() -> TResult<()> {
        let dt: DateTime = "2023-05-15 14:30:45".parse()?;
        assert_eq!(dt.year(), Some(2023));
        assert_eq!(dt.month(), Some(5));
        assert_eq!(dt.day(), Some(15));
        assert_eq!(dt.hour(), Some(14));
        assert_eq!(dt.minute(), Some(30));
        assert_eq!(dt.second(), Some(45));
        Ok(())
    }

    #[test]
    fn test_nat_datetime() {
        let nat_dt: DateTime = DateTime::nat();
        assert!(nat_dt.is_nat());
        assert_eq!(nat_dt.year(), None);
        assert_eq!(nat_dt.month(), None);
        assert_eq!(nat_dt.day(), None);
        assert_eq!(nat_dt.hour(), None);
        assert_eq!(nat_dt.minute(), None);
        assert_eq!(nat_dt.second(), None);
    }

    #[test]
    fn test_invalid_datetime_parse() {
        assert!("invalid date".parse::<DateTime>().is_err());
    }
}
