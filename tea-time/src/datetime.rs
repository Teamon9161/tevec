use std::{cmp::Ordering, hash::Hash, marker::PhantomData};

use super::timeunit::*;
use crate::TimeDelta;
use chrono::{
    DateTime as CrDateTime, Datelike, DurationRound, Months, NaiveDateTime, NaiveTime, Timelike,
    Utc,
};

use tea_error::{tbail, terr, TResult};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DateTime<U: TimeUnitTrait = Nanosecond>(pub i64, PhantomData<U>);

const TIME_RULE_VEC: [&str; 9] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M:%S.%f",
    "%Y-%m-%d",
    "%Y%m%d",
    "%Y%m%d %H%M%S",
    "%d/%m/%Y",
    "%d/%m/%Y H%M%S",
    "%Y%m%d%H%M%S",
    "%d/%m/%YH%M%S",
];

impl<U: TimeUnitTrait> DateTime<U> {
    #[inline]
    pub fn new(dt: i64) -> Self {
        Self(dt, PhantomData)
    }

    #[inline]
    pub fn is_nat(&self) -> bool {
        self.0 == i64::MIN
    }

    #[inline]
    pub fn is_not_nat(&self) -> bool {
        self.0 != i64::MIN
    }

    #[inline]
    pub fn nat() -> Self {
        Self(i64::MIN, PhantomData)
    }

    #[inline]
    pub fn into_i64(self) -> i64 {
        self.0
    }

    #[inline]
    pub fn into_opt_i64(self) -> Option<i64> {
        if self.is_nat() {
            None
        } else {
            Some(self.0)
        }
    }

    #[inline]
    pub fn to_cr(&self) -> Option<CrDateTime<Utc>>
    where
        Self: TryInto<CrDateTime<Utc>>,
    {
        if self.is_nat() {
            None
        } else {
            (*self).try_into().ok()
        }
    }

    #[inline(always)]
    pub fn parse(s: &str, fmt: Option<&str>) -> TResult<Self>
    where
        Self: From<CrDateTime<Utc>>,
    {
        if let Some(fmt) = fmt {
            let cr_dt = NaiveDateTime::parse_from_str(s, fmt)
                .map_err(|err| terr!(ParseError:"Failed to parse datetime: {}", err))?;
            Ok(cr_dt.into())
        } else {
            for fmt in TIME_RULE_VEC.iter() {
                if let Ok(cr_dt) = NaiveDateTime::parse_from_str(s, fmt) {
                    return Ok(cr_dt.into());
                }
            }
            tbail!(ParseError:"Failed to parse datetime from string: {}", s)
        }
    }

    #[inline]
    pub fn strftime(&self, fmt: Option<&str>) -> String
    where
        Self: TryInto<CrDateTime<Utc>>,
        <Self as TryInto<CrDateTime<Utc>>>::Error: std::fmt::Debug,
    {
        if self.is_nat() {
            "NaT".to_string()
        } else {
            let fmt = fmt.unwrap_or("%Y-%m-%d %H:%M:%S.%f");
            self.to_cr().unwrap().format(fmt).to_string()
        }
    }

    pub fn duration_trunc(self, duration: TimeDelta) -> Self
    where
        Self: TryInto<CrDateTime<Utc>> + From<CrDateTime<Utc>>,
        <Self as TryInto<CrDateTime<Utc>>>::Error: std::fmt::Debug,
    {
        if self.is_nat() {
            return self;
        }
        let mut dt = self.to_cr().unwrap();
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
    // <Self as TryInto<CrDateTime<Utc>>>::Error: std::fmt::Debug,
{
    #[inline(always)]
    pub fn time(&self) -> Option<NaiveTime> {
        self.to_cr().map(|dt| dt.time())
    }

    #[inline(always)]
    pub fn day(&self) -> Option<usize> {
        self.to_cr().map(|dt| dt.day() as usize)
    }

    #[inline(always)]
    pub fn month(&self) -> Option<usize> {
        self.to_cr().map(|dt| dt.month() as usize)
    }

    #[inline(always)]
    pub fn hour(&self) -> Option<usize> {
        self.to_cr().map(|dt| dt.hour() as usize)
    }

    #[inline(always)]
    pub fn minute(&self) -> Option<usize> {
        self.to_cr().map(|dt| dt.minute() as usize)
    }

    #[inline(always)]
    pub fn second(&self) -> Option<usize> {
        self.to_cr().map(|dt| dt.second() as usize)
    }
}
