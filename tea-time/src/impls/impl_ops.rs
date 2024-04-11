use crate::{DateTime, TimeDelta};
use chrono::Months;
use std::ops::{Add, Sub};

impl Add<TimeDelta> for DateTime {
    type Output = DateTime;
    fn add(self, rhs: TimeDelta) -> Self::Output {
        if let Some(dt) = self.0 && rhs.is_not_nat() {
            let out = if rhs.months != 0 {
                if rhs.months > 0 {
                    dt + Months::new(rhs.months as u32)
                } else {
                    dt - Months::new((-rhs.months) as u32)
                }
            } else {
                dt
            };
            DateTime(Some(out + rhs.inner))
        } else {
            DateTime(None)
        }
    }
}

impl Sub<TimeDelta> for DateTime {
    type Output = DateTime;
    fn sub(self, rhs: TimeDelta) -> Self::Output {
        if let Some(dt) = self.0 && rhs.is_not_nat(){
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
