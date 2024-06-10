use std::cmp::Ordering;

use crate::TimeDelta;
use chrono::Duration;

impl Default for TimeDelta {
    #[inline(always)]
    fn default() -> Self {
        TimeDelta::nat()
    }
}

impl From<Duration> for TimeDelta {
    #[inline(always)]
    fn from(duration: Duration) -> Self {
        Self {
            months: 0,
            inner: duration,
        }
    }
}

impl From<i64> for TimeDelta {
    #[inline]
    fn from(dt: i64) -> Self {
        if dt == i64::MIN {
            return TimeDelta::nat();
        }
        Duration::microseconds(dt).into()
    }
}

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
