use std::cmp::Ordering;

use chrono::Duration;

use crate::TimeDelta;

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

impl From<Option<Duration>> for TimeDelta {
    #[inline]
    fn from(duration: Option<Duration>) -> Self {
        if let Some(duration) = duration {
            Self::from(duration)
        } else {
            Self::nat()
        }
    }
}

impl From<i64> for TimeDelta {
    #[inline]
    fn from(dt: i64) -> Self {
        if dt == i64::MIN {
            return TimeDelta::nat();
        }
        Duration::nanoseconds(dt).into()
    }
}

impl From<Option<i64>> for TimeDelta {
    #[inline]
    fn from(dt: Option<i64>) -> Self {
        if let Some(dt) = dt {
            Self::from(dt)
        } else {
            Self::nat()
        }
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
