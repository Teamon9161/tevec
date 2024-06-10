use crate::{DateTime, TimeDelta, TimeUnitTrait};
use chrono::{DateTime as CrDateTime, Months, Utc};
use std::ops::{Add, Div, Mul, Neg, Sub};

// TODO: improve performance for time operation

impl<U: TimeUnitTrait> Add<TimeDelta> for DateTime<U>
where
    Self: From<CrDateTime<Utc>> + TryInto<CrDateTime<Utc>>,
{
    type Output = DateTime<U>;
    fn add(self, rhs: TimeDelta) -> Self::Output {
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt = self.to_cr().unwrap();
            let out = if rhs.months != 0 {
                if rhs.months > 0 {
                    dt + Months::new(rhs.months as u32)
                } else {
                    dt - Months::new((-rhs.months) as u32)
                }
            } else {
                dt
            };
            (out + rhs.inner).try_into().unwrap()
        } else {
            DateTime::nat()
        }
    }
}

impl<U: TimeUnitTrait> Sub<TimeDelta> for DateTime<U>
where
    Self: From<CrDateTime<Utc>> + TryInto<CrDateTime<Utc>>,
{
    type Output = DateTime;
    fn sub(self, rhs: TimeDelta) -> Self::Output {
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt = self.to_cr().unwrap();
            let out = if rhs.months != 0 {
                if rhs.months > 0 {
                    dt - Months::new(rhs.months as u32)
                } else {
                    dt + Months::new((-rhs.months) as u32)
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

impl<U: TimeUnitTrait> Sub<DateTime<U>> for DateTime<U>
where
    Self: From<CrDateTime<Utc>> + TryInto<CrDateTime<Utc>>,
{
    type Output = TimeDelta;
    fn sub(self, rhs: DateTime<U>) -> Self::Output {
        // TODO: improve performance
        // this can be done by implement unit conversion
        if self.is_not_nat() && rhs.is_not_nat() {
            let dt1 = self.to_cr().unwrap();
            let dt2 = rhs.to_cr().unwrap();
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
