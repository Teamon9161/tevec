use chrono::{DateTime as CrDateTime, Utc};
use crate::DateTime;
use std::ops::Deref;

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(dt) = self.0 {
            write!(f, "{dt}")
        } else {
            write!(f, "None")
        }
    }
}

impl Deref for DateTime {
    type Target = Option<CrDateTime<Utc>>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Option<CrDateTime<Utc>>> for DateTime {
    #[inline(always)]
    fn from(dt: Option<CrDateTime<Utc>>) -> Self {
        Self(dt)
    }
}

impl From<CrDateTime<Utc>> for DateTime {
    #[inline(always)]
    fn from(dt: CrDateTime<Utc>) -> Self {
        Self(Some(dt))
    }
}

impl From<i64> for DateTime {
    #[inline]
    fn from(dt: i64) -> Self {
        if dt == i64::MIN {
            return DateTime(None);
        }
        // DateTime::from_timestamp_us(dt).unwrap_or_default()
        DateTime::from_timestamp_ns(dt).unwrap_or_default()
    }
}

impl ToString for DateTime {
    #[inline(always)]
    fn to_string(&self) -> String {
        self.strftime(None)
    }
}
