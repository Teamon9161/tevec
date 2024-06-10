use crate::*;
use chrono::{DateTime as CrDateTime, NaiveDateTime, Utc};
use std::{convert::TryFrom, str::FromStr};
use tea_error::*;

impl<U: TimeUnitTrait> From<i64> for DateTime<U> {
    #[inline]
    fn from(dt: i64) -> Self {
        DateTime::new(dt)
    }
}

impl<U: TimeUnitTrait> FromStr for DateTime<U>
where
    Self: From<CrDateTime<Utc>>,
{
    type Err = TError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse(s, None)
    }
}

impl<U: TimeUnitTrait> From<NaiveDateTime> for DateTime<U>
where
    Self: From<CrDateTime<Utc>>,
{
    #[inline]
    fn from(dt: NaiveDateTime) -> Self {
        CrDateTime::from_naive_utc_and_offset(dt, Utc).into()
    }
}

impl From<CrDateTime<Utc>> for DateTime<Second> {
    #[inline]
    fn from(dt: CrDateTime<Utc>) -> Self {
        dt.timestamp().into()
    }
}

impl TryFrom<DateTime<Second>> for CrDateTime<Utc> {
    type Error = TError;
    #[inline]
    fn try_from(dt: DateTime<Second>) -> TResult<Self> {
        CrDateTime::from_timestamp(dt.0, 0)
            .ok_or_else(|| terr!("Failed to convert DateTime<Second> to CrDateTime"))
    }
}

impl TryFrom<DateTime<Millisecond>> for CrDateTime<Utc> {
    type Error = TError;
    #[inline]
    fn try_from(dt: DateTime<Millisecond>) -> TResult<Self> {
        CrDateTime::from_timestamp_millis(dt.0)
            .ok_or_else(|| terr!("Failed to convert DateTime<Millisecond> to CrDateTime"))
    }
}

impl TryFrom<DateTime<Microsecond>> for CrDateTime<Utc> {
    type Error = TError;
    #[inline]
    fn try_from(dt: DateTime<Microsecond>) -> TResult<Self> {
        CrDateTime::from_timestamp_micros(dt.0)
            .ok_or_else(|| terr!("Failed to convert DateTime<Microsecond> to CrDateTime"))
    }
}

impl TryFrom<DateTime<Nanosecond>> for CrDateTime<Utc> {
    type Error = TError;
    #[inline]
    fn try_from(dt: DateTime<Nanosecond>) -> TResult<Self> {
        Ok(CrDateTime::from_timestamp_nanos(dt.0))
    }
}

impl From<CrDateTime<Utc>> for DateTime<Millisecond> {
    #[inline]
    fn from(dt: CrDateTime<Utc>) -> Self {
        dt.timestamp_millis().into()
    }
}

impl From<CrDateTime<Utc>> for DateTime<Microsecond> {
    #[inline]
    fn from(dt: CrDateTime<Utc>) -> Self {
        dt.timestamp_micros().into()
    }
}

impl From<CrDateTime<Utc>> for DateTime<Nanosecond> {
    #[inline]
    fn from(dt: CrDateTime<Utc>) -> Self {
        dt.timestamp_nanos_opt()
            .expect("Failed to convert to nanosecond")
            .into()
    }
}
