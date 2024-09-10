use std::convert::TryFrom;
use std::str::FromStr;

use chrono::{DateTime as CrDateTime, NaiveDate, NaiveDateTime, Utc};
use tea_error::*;

use crate::*;

// const fn try_into_chrono_datetime<U: TimeUnitTrait>(dt: DateTime<U>) -> CrDateTime<Utc> {
//     match U::unit() {
//         TimeUnit::Second => CrDateTime::from_timestamp(dt.0, 0).unwrap(),
//         TimeUnit::Millisecond => CrDateTime::from_timestamp_millis(dt.0).unwrap(),
//         TimeUnit::Microsecond => CrDateTime::from_timestamp_micros(dt.0).unwrap(),
//         TimeUnit::Nanosecond => CrDateTime::from_timestamp_nanos(dt.0),
//         _ => todo!(),
//     }
// }

impl<U: TimeUnitTrait> From<i64> for DateTime<U> {
    #[inline]
    fn from(dt: i64) -> Self {
        DateTime::new(dt)
    }
}

impl<U: TimeUnitTrait> Default for DateTime<U> {
    #[inline]
    fn default() -> Self {
        DateTime::nat()
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

impl<U: TimeUnitTrait> From<Option<NaiveDateTime>> for DateTime<U>
where
    Self: From<CrDateTime<Utc>>,
{
    #[inline]
    fn from(dt: Option<NaiveDateTime>) -> Self {
        if let Some(dt) = dt {
            CrDateTime::from_naive_utc_and_offset(dt, Utc).into()
        } else {
            DateTime::nat()
        }
    }
}

impl<U: TimeUnitTrait> From<NaiveDate> for DateTime<U>
where
    Self: From<CrDateTime<Utc>>,
{
    #[inline]
    fn from(dt: NaiveDate) -> Self {
        CrDateTime::from_naive_utc_and_offset(dt.and_hms_opt(0, 0, 0).unwrap(), Utc).into()
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

impl From<CrDateTime<Utc>> for DateTime<Second> {
    #[inline]
    fn from(dt: CrDateTime<Utc>) -> Self {
        dt.timestamp().into()
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
