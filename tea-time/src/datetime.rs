// use ndarray::ScalarOperand;
use std::{
    cmp::Ordering,
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use chrono::{DateTime as CrDateTime, Datelike, DurationRound, Months, NaiveTime, Timelike, Utc};

use crate::{convert::*, TimeDelta};

use tea_error::{tbail, terr, TError, TResult};

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, PartialOrd)]
pub struct DateTime(pub Option<CrDateTime<Utc>>);

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

impl FromStr for DateTime {
    type Err = TError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse(s, None)
    }
}

impl DateTime {
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub fn into_i64(self) -> i64 {
        self.0
            .map_or(i64::MIN, |dt| dt.timestamp_nanos_opt().unwrap_or(i64::MIN))
    }

    #[inline]
    pub fn from_timestamp_opt(secs: i64, nsecs: u32) -> Self {
        Self(CrDateTime::from_timestamp(secs, nsecs))
    }

    #[inline]
    pub fn from_timestamp_ms(ms: i64) -> Option<Self> {
        let mut secs = ms / MILLIS_PER_SEC;
        if ms < 0 {
            secs = secs.checked_sub(1)?;
        }

        let nsecs = (ms % MILLIS_PER_SEC).abs();
        let nsecs = if nsecs == 0 && ms < 0 {
            secs += 1;
            0
        } else {
            let mut nsecs = u32::try_from(nsecs).ok()? * NANOS_PER_MILLI as u32;
            if secs < 0 {
                nsecs = (NANOS_PER_SEC as u32).checked_sub(nsecs)?;
            }
            nsecs
        };
        Some(Self::from_timestamp_opt(secs, nsecs))
    }

    #[inline]
    pub fn from_timestamp_us(us: i64) -> Option<Self> {
        let mut secs = us / MICROS_PER_SEC;
        if us < 0 {
            secs = secs.checked_sub(1)?;
        }

        let nsecs = (us % MICROS_PER_SEC).abs();
        let nsecs = if nsecs == 0 && us < 0 {
            secs += 1;
            0
        } else {
            let mut nsecs = u32::try_from(nsecs).ok()? * NANOS_PER_MICRO as u32;
            if secs < 0 {
                nsecs = (NANOS_PER_SEC as u32).checked_sub(nsecs)?;
            }
            nsecs
        };
        Some(Self::from_timestamp_opt(secs, nsecs))
    }

    #[inline]
    pub fn from_timestamp_ns(ns: i64) -> Option<Self> {
        let mut secs = ns / NANOS_PER_SEC;
        if ns < 0 {
            secs = secs.checked_sub(1)?;
        }

        let nsecs = (ns % NANOS_PER_SEC).abs();
        let nsecs = if nsecs == 0 && ns < 0 {
            secs += 1;
            0
        } else {
            let mut nsecs = u32::try_from(nsecs).ok()?;
            if secs < 0 {
                nsecs = (NANOS_PER_SEC as u32).checked_sub(nsecs)?;
            }
            nsecs
        };
        Some(Self::from_timestamp_opt(secs, nsecs))
    }

    #[inline(always)]
    pub fn parse(s: &str, fmt: Option<&str>) -> TResult<Self> {
        if let Some(fmt) = fmt {
            let cr_dt = CrDateTime::parse_from_str(s, fmt)
                .map_err(|err| terr!(ParseError:"Failed to parse datetime: {}", err))?;
            Ok(Self(Some(cr_dt.into())))
        } else {
            for fmt in TIME_RULE_VEC.iter() {
                if let Ok(cr_dt) = CrDateTime::parse_from_str(s, fmt) {
                    return Ok(Self(Some(cr_dt.into())));
                }
            }
            tbail!(ParseError:"Failed to parse datetime from string: {}", s)
        }
    }

    #[inline]
    pub fn strftime(&self, fmt: Option<&str>) -> String {
        if let Some(fmt) = fmt {
            self.0
                .map_or("NaT".to_string(), |dt| dt.format(fmt).to_string())
        } else {
            self.0.map_or("NaT".to_string(), |dt| dt.to_string())
        }
    }

    pub fn duration_trunc(self, duration: TimeDelta) -> Self {
        if self.is_none() {
            return self;
        }
        let mut dt = self.0.unwrap();
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

// #[pyclass]
// pub struct PyDateTime(DateTime);

// impl ToPyObject for DateTime {
//     #[inline(always)]
//     fn to_object(&self, py: Python<'_>) -> PyObject {
//         PyDateTime(*self).into_py(py)
//     }
// }

impl DateTime {
    // #[inline]
    // pub fn into_np_datetime<T: NPUnit>(self) -> NPDatetime<T> {
    //     use NPY_DATETIMEUNIT::*;
    //     if let Some(dt) = self.0 {
    //         match T::UNIT {
    //             NPY_FR_ms => dt.timestamp_millis().into(),
    //             NPY_FR_us => dt.timestamp_micros().into(),
    //             NPY_FR_ns => dt.timestamp_nanos_opt().unwrap_or(i64::MIN).into(),
    //             _ => unreachable!(),
    //         }
    //     } else {
    //         i64::MIN.into()
    //     }
    // }

    #[inline(always)]
    pub fn time(&self) -> Option<NaiveTime> {
        self.0.map(|dt| dt.time())
    }

    #[inline(always)]
    pub fn day(&self) -> Option<usize> {
        self.0.map(|dt| dt.day() as usize)
    }

    #[inline(always)]
    pub fn month(&self) -> Option<usize> {
        self.0.map(|dt| dt.month() as usize)
    }

    #[inline(always)]
    pub fn hour(&self) -> Option<usize> {
        self.0.map(|dt| dt.hour() as usize)
    }

    #[inline(always)]
    pub fn minute(&self) -> Option<usize> {
        self.0.map(|dt| dt.minute() as usize)
    }

    #[inline(always)]
    pub fn second(&self) -> Option<usize> {
        self.0.map(|dt| dt.second() as usize)
    }
}

impl Neg for TimeDelta {
    type Output = TimeDelta;

    #[inline]
    fn neg(self) -> TimeDelta {
        if self.is_not_nat() {
            Self {
                months: -self.months,
                inner: -self.inner,
            }
        } else {
            self
        }
    }
}

impl Add for TimeDelta {
    type Output = TimeDelta;
    #[inline]
    fn add(self, rhs: TimeDelta) -> TimeDelta {
        if self.is_not_nat() & rhs.is_not_nat() {
            Self {
                months: self.months + rhs.months,
                inner: self.inner + rhs.inner,
            }
        } else {
            TimeDelta::nat()
        }
    }
}

impl Sub for TimeDelta {
    type Output = TimeDelta;
    #[inline]
    fn sub(self, rhs: TimeDelta) -> TimeDelta {
        if self.is_not_nat() & rhs.is_not_nat() {
            Self {
                months: self.months - rhs.months,
                inner: self.inner - rhs.inner,
            }
        } else {
            TimeDelta::nat()
        }
    }
}

impl Mul<i32> for TimeDelta {
    type Output = TimeDelta;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        if self.is_not_nat() {
            Self {
                months: self.months * rhs,
                inner: self.inner * rhs,
            }
        } else {
            TimeDelta::nat()
        }
    }
}

impl Div<TimeDelta> for TimeDelta {
    type Output = i32;

    fn div(self, rhs: TimeDelta) -> Self::Output {
        if self.is_not_nat() & rhs.is_not_nat() {
            // may not as expected
            let inner_div =
                self.inner.num_nanoseconds().unwrap() / rhs.inner.num_nanoseconds().unwrap();
            if self.months == 0 || rhs.months == 0 {
                return inner_div as i32;
            }
            let month_div = self.months / rhs.months;
            if month_div == inner_div as i32 {
                month_div
            } else {
                panic!("not support div TimeDelta when month div and time div is not equal")
            }
        } else {
            panic!("not support div TimeDelta when one of them is nat")
        }
    }
}

// impl PartialEq for TimeDelta {
//     fn eq(&self, other: &Self) -> bool {
//         if self.months != other.months {
//             false
//         } else {
//             self.inner.eq(&other.inner)
//         }
//     }
// }

impl PartialOrd for TimeDelta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_not_nat() {
            // may not as expected
            if self.months != other.months {
                self.months.partial_cmp(&other.months)
            } else {
                self.inner.partial_cmp(&other.inner)
            }
        } else {
            None
        }
    }
}

// impl ScalarOperand for TimeDelta {}
// impl ScalarOperand for DateTime {}

// impl From<&str> for TimeDelta {
//     #[inline(always)]
//     fn from(s: &str) -> Self {
//         TimeDelta::parse(s)
//     }
// }
