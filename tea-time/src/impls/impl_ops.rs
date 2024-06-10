use crate::{DateTime, TimeDelta, TimeUnit};
use chrono::{DateTime as CrDateTime, Months, Utc};
use std::ops::{Add, Sub};

impl<U: TimeUnit> Add<TimeDelta> for DateTime<U> {
    type Output = DateTime<U>;
    fn add(self, rhs: TimeDelta) -> Self::Output {
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt: CrDateTime<Utc> = self.into();
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

impl Sub<TimeDelta> for DateTime {
    type Output = DateTime;
    fn sub(self, rhs: TimeDelta) -> Self::Output {
        if let Some(dt) = self.0 {
            if rhs.is_not_nat() {
                let out = if rhs.months != 0 {
                    if rhs.months > 0 {
                        dt - Months::new(rhs.months as u32)
                    } else {
                        dt + Months::new((-rhs.months) as u32)
                    }
                } else {
                    dt
                };
                DateTime(Some(out + rhs.inner))
            } else {
                DateTime(None)
            }
        } else {
            DateTime(None)
        }
    }
}

impl Sub<DateTime> for DateTime {
    type Output = TimeDelta;
    fn sub(self, rhs: DateTime) -> Self::Output {
        if let (Some(dt1), Some(dt2)) = (self.0, rhs.0) {
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
