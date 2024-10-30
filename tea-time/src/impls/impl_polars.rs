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

impl From<DateTime<Nanosecond>> for AnyValue<'_> {
    #[inline]
    fn from(value: DateTime<Nanosecond>) -> Self {
        if value.is_nat() {
            AnyValue::Null
        } else {
            AnyValue::Datetime(value.0, TimeUnit::Nanoseconds, None)
        }
    }
}

impl From<DateTime<Microsecond>> for AnyValue<'_> {
    #[inline]
    fn from(value: DateTime<Microsecond>) -> Self {
        if value.is_nat() {
            AnyValue::Null
        } else {
            AnyValue::Datetime(value.0, TimeUnit::Microseconds, None)
        }
    }
}

impl From<DateTime<Millisecond>> for AnyValue<'_> {
    #[inline]
    fn from(value: DateTime<Millisecond>) -> Self {
        if value.is_nat() {
            AnyValue::Null
        } else {
            AnyValue::Datetime(value.0, TimeUnit::Milliseconds, None)
        }
    }
}

impl From<Time> for AnyValue<'_> {
    #[inline]
    fn from(value: Time) -> Self {
        if value.is_nat() {
            AnyValue::Null
        } else {
            AnyValue::Time(value.0)
        }
    }
}

impl From<TimeDelta> for AnyValue<'_> {
    #[inline]
    #[allow(clippy::collapsible_else_if)]
    fn from(value: TimeDelta) -> Self {
        use crate::convert::*;
        // if nanoseconds part is 0, we convert to microseconds unit
        if value.is_nat() {
            AnyValue::Null
        } else {
            let sub_sec_nanos = value.inner.subsec_nanos();
            let remain_nanos = sub_sec_nanos % NANOS_PER_MICRO as i32;
            if remain_nanos != 0 {
                if value.months == 0 {
                    AnyValue::Duration(
                        value.inner.num_nanoseconds().unwrap(),
                        TimeUnit::Nanoseconds,
                    )
                } else {
                    // follow `polars_time::windows::duration::Duration::duration_ns` implementation
                    let month_nanos = value.months as i64 * 28 * 24 * 3600 * NANOS_PER_SEC;
                    let nanos = value.inner.num_nanoseconds().unwrap();

                    AnyValue::Duration(month_nanos + nanos, TimeUnit::Nanoseconds)
                }
            } else {
                if value.months == 0 {
                    AnyValue::Duration(
                        value.inner.num_microseconds().unwrap(),
                        TimeUnit::Microseconds,
                    )
                } else {
                    let month_micros = value.months as i64 * 28 * 24 * 3600 * MICROS_PER_SEC;
                    let micros = value.inner.num_microseconds().unwrap();

                    AnyValue::Duration(month_micros + micros, TimeUnit::Microseconds)
                }
            }
        }
    }
}
