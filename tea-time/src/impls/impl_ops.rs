use std::ops::{Add, Div, Mul, Neg, Sub};

use chrono::{DateTime as CrDateTime, Months, Utc};

use crate::{DateTime, Time, TimeDelta, TimeUnitTrait};

// TODO: improve performance for time operation

impl<U: TimeUnitTrait> Add<TimeDelta> for DateTime<U>
where
    Self: From<CrDateTime<Utc>> + TryInto<CrDateTime<Utc>>,
{
    type Output = DateTime<U>;
    fn add(self, rhs: TimeDelta) -> Self::Output {
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt = self.as_cr().unwrap();
            let out = if rhs.months != 0 {
                if rhs.months > 0 {
                    dt + Months::new(rhs.months as u32)
                } else {
                    dt - Months::new((-rhs.months) as u32)
                }
            } else {
                dt
            };
            (out + rhs.inner).into()
        } else {
            DateTime::nat()
        }
    }
}

impl<U: TimeUnitTrait> Sub<TimeDelta> for DateTime<U>
where
    Self: From<CrDateTime<Utc>> + TryInto<CrDateTime<Utc>>,
{
    type Output = DateTime<U>;
    fn sub(self, rhs: TimeDelta) -> Self::Output {
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt = self.as_cr().unwrap();
            let out = if rhs.months != 0 {
                if rhs.months > 0 {
                    dt - Months::new(rhs.months as u32)
                } else {
                    dt + Months::new((-rhs.months) as u32)
                }
            } else {
                dt
            };
            (out - rhs.inner).into()
        } else {
            DateTime::nat()
        }
    }
}

impl<U: TimeUnitTrait> Sub<DateTime<U>> for DateTime<U>
where
    Self: From<CrDateTime<Utc>> + TryInto<CrDateTime<Utc>>,
{
    type Output = TimeDelta;
    fn sub(self, rhs: DateTime<U>) -> Self::Output {
        // TODO: improve performance
        // this can be done by implement unit conversion
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt1 = self.as_cr().unwrap();
            let dt2 = rhs.as_cr().unwrap();
            let duration = dt1 - dt2;
            TimeDelta {
                months: 0,
                inner: duration,
            }
            // let r_year = dt2.year();
            // let years = dt1.year() - r_year;
            // let months = dt1.month() as i32 - dt2.month() as i32;
            // let duration =
            //     dt1.with_year(r_year).expect(&format!("{dt1} with {r_year}")).with_month(1).expect(&format!("{dt1} with month1")) - dt2.with_month(1).expect(&format!("{dt2} with month1"));
            // TimeDelta {
            //     months: 12 * years + months,
            //     inner: duration,
            // }
        } else {
            TimeDelta::nat()
        }
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

impl Add<TimeDelta> for Time {
    type Output = Time;
    fn add(self, rhs: TimeDelta) -> Self::Output {
        if rhs.is_not_nat() {
            if rhs.months != 0 {
                panic!("not support add TimeDelta with months");
            }
            if let Some(nanos) = rhs.inner.num_nanoseconds() {
                let nanos = self.0 + nanos;
                return Time(nanos);
            }
        }
        Time::nat()
    }
}

impl Sub<TimeDelta> for Time {
    type Output = Time;
    fn sub(self, rhs: TimeDelta) -> Self::Output {
        if rhs.is_not_nat() {
            if rhs.months != 0 {
                panic!("not support sub TimeDelta with months");
            }
            if let Some(nanos) = rhs.inner.num_nanoseconds() {
                let nanos = self.0 - nanos;
                return Time(nanos);
            }
        }
        Time::nat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TimeDelta;

    #[test]
    fn test_time_add_timedelta() {
        let time = Time::from_hms(12, 0, 0);
        let delta = TimeDelta::parse("1h30m").unwrap();
        let result = time + delta;
        assert_eq!(result, Time::from_hms(13, 30, 0));
    }

    #[test]
    fn test_time_sub_timedelta() {
        let time = Time::from_hms(12, 0, 0);
        let delta = TimeDelta::parse("1h30m").unwrap();
        let result = time - delta;
        assert_eq!(result, Time::from_hms(10, 30, 0));
    }

    #[test]
    #[should_panic]
    fn test_time_add_timedelta_with_months() {
        let time = Time::from_hms(12, 0, 0);
        let delta = TimeDelta::parse("1mo1h30m").unwrap();
        let _ = time + delta;
    }

    #[test]
    #[should_panic]
    fn test_time_sub_timedelta_with_months() {
        let time = Time::from_hms(12, 0, 0);
        let delta = TimeDelta::parse("1mo1h30m").unwrap();
        let _ = time - delta;
    }

    #[test]
    fn test_time_add_nat_timedelta() {
        let time = Time::from_hms(12, 0, 0);
        let nat_delta = TimeDelta::nat();
        let result = time + nat_delta;
        assert_eq!(result, Time::nat());
    }

    #[test]
    fn test_time_sub_nat_timedelta() {
        let time = Time::from_hms(12, 0, 0);
        let nat_delta = TimeDelta::nat();
        let result = time - nat_delta;
        assert_eq!(result, Time::nat());
    }
}
