#[allow(unused_imports)]
use tea_deps::polars::prelude::{DataType as PlDataType, *};

use crate::*;

#[cfg(feature = "time")]
impl Cast<DateTime<unit::Nanosecond>> for AnyValue<'_> {
    #[inline]
    fn cast(self) -> DateTime<unit::Nanosecond> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl Cast<DateTime<unit::Microsecond>> for AnyValue<'_> {
    #[inline]
    fn cast(self) -> DateTime<unit::Microsecond> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl Cast<DateTime<unit::Millisecond>> for AnyValue<'_> {
    #[inline]
    fn cast(self) -> DateTime<unit::Millisecond> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl<'a> Cast<AnyValue<'a>> for DateTime<unit::Nanosecond> {
    #[inline]
    fn cast(self) -> AnyValue<'a> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl<'a> Cast<AnyValue<'a>> for DateTime<unit::Microsecond> {
    #[inline]
    fn cast(self) -> AnyValue<'a> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl<'a> Cast<AnyValue<'a>> for DateTime<unit::Millisecond> {
    #[inline]
    fn cast(self) -> AnyValue<'a> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for AnyValue<'_> {
    #[inline]
    fn cast(self) -> TimeDelta {
        self.into()
    }
}

#[cfg(feature = "time")]
impl<'a> Cast<AnyValue<'a>> for TimeDelta {
    #[inline]
    fn cast(self) -> AnyValue<'a> {
        self.into()
    }
}

#[cfg(feature = "time")]
impl Cast<Time> for AnyValue<'_> {
    #[inline]
    fn cast(self) -> Time {
        self.into()
    }
}

#[cfg(feature = "time")]
impl<'a> Cast<AnyValue<'a>> for Time {
    #[inline]
    fn cast(self) -> AnyValue<'a> {
        self.into()
    }
}
