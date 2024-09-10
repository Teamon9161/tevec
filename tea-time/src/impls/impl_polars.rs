#[cfg(feature = "polars-lazy")]
use tea_deps::polars::lazy::dsl::Expr;
use tea_deps::polars::prelude::*;

use crate::{DateTime, Microsecond, Millisecond, Nanosecond, Time, TimeDelta};

#[cfg(feature = "polars-lazy")]
impl Literal for Time {
    #[inline]
    fn lit(self) -> Expr {
        LiteralValue::Time(self.0).lit()
    }
}

#[cfg(feature = "polars-lazy")]
impl Literal for DateTime<Nanosecond> {
    #[inline]
    fn lit(self) -> Expr {
        LiteralValue::DateTime(self.0, TimeUnit::Nanoseconds, None).lit()
    }
}

#[cfg(feature = "polars-lazy")]
impl Literal for DateTime<Microsecond> {
    #[inline]
    fn lit(self) -> Expr {
        LiteralValue::DateTime(self.0, TimeUnit::Microseconds, None).lit()
    }
}

#[cfg(feature = "polars-lazy")]
impl Literal for DateTime<Millisecond> {
    #[inline]
    fn lit(self) -> Expr {
        LiteralValue::DateTime(self.0, TimeUnit::Milliseconds, None).lit()
    }
}

#[cfg(feature = "polars-lazy")]
impl Literal for TimeDelta {
    #[inline]
    fn lit(self) -> Expr {
        use tea_deps::polars::time::Duration;
        lit(self.inner) + lit(Duration::parse(&format!("{}mo", self.months)))
    }
}

impl<'a> From<AnyValue<'a>> for DateTime<Nanosecond> {
    #[inline]
    fn from(value: AnyValue<'a>) -> Self {
        match value.dtype() {
            DataType::Datetime(TimeUnit::Nanoseconds, None) => value.extract::<i64>().into(),
            _ => value
                .cast(&DataType::Datetime(TimeUnit::Nanoseconds, None))
                .extract::<i64>()
                .into(),
        }
    }
}

impl<'a> From<AnyValue<'a>> for DateTime<Microsecond> {
    #[inline]
    fn from(value: AnyValue<'a>) -> Self {
        match value.dtype() {
            DataType::Datetime(TimeUnit::Microseconds, None) => value.extract::<i64>().into(),
            _ => value
                .cast(&DataType::Datetime(TimeUnit::Microseconds, None))
                .extract::<i64>()
                .into(),
        }
    }
}

impl<'a> From<AnyValue<'a>> for DateTime<Millisecond> {
    #[inline]
    fn from(value: AnyValue<'a>) -> Self {
        match value.dtype() {
            DataType::Datetime(TimeUnit::Milliseconds, None) => value.extract::<i64>().into(),
            _ => value
                .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))
                .extract::<i64>()
                .into(),
        }
    }
}

impl<'a> From<AnyValue<'a>> for Time {
    #[inline]
    fn from(value: AnyValue<'a>) -> Self {
        let dt = value.cast(&DataType::Time).extract::<i64>();
        dt.into()
    }
}

impl<'a> From<AnyValue<'a>> for TimeDelta {
    #[inline]
    fn from(value: AnyValue<'a>) -> Self {
        use tea_deps::chrono::Duration as CrDuration;
        match value.dtype() {
            DataType::Duration(TimeUnit::Nanoseconds) => {
                value.extract::<i64>().map(CrDuration::nanoseconds).into()
            },
            DataType::Duration(TimeUnit::Microseconds) => {
                value.extract::<i64>().map(CrDuration::microseconds).into()
            },
            DataType::Duration(TimeUnit::Milliseconds) => {
                value.extract::<i64>().map(CrDuration::milliseconds).into()
            },
            _ => {
                let nanos = value
                    .cast(&DataType::Duration(TimeUnit::Nanoseconds))
                    .extract::<i64>();
                nanos.map(CrDuration::nanoseconds).into()
            },
        }
    }
}
