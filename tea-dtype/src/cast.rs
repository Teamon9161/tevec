#[cfg(feature = "time")]
use tea_time::{DateTime, TimeDelta};

use super::isnone::IsNone;

pub trait Cast<T> {
    fn cast(self) -> T;
}

impl<T: IsNone> Cast<T> for Option<T> {
    #[inline]
    fn cast(self) -> T {
        self.unwrap_or(T::none())
    }
}

impl<T: IsNone> Cast<Option<T>> for T {
    #[inline]
    fn cast(self) -> Option<T> {
        if self.is_none() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> Cast<T> for T {
    #[inline(always)]
    fn cast(self) -> T {
        self
    }
}

macro_rules! impl_numeric_cast {
    (@ $T: ty => $(#[$cfg:meta])* impl $U: ty ) => {
        $(#[$cfg])*
        impl Cast<$U> for $T {
            #[inline] fn cast(self) -> $U { self as $U }
        }
    };

    (@ $T: ty => { $( $U: ty ),* } ) => {$(
        impl_numeric_cast!(@ $T => impl $U);
    )*};

    (@common_impl $T: ty => { $( $U: ty ),* } ) => {
        impl Cast<String> for $T {
            #[inline] fn cast(self) -> String { self.to_string() }
        }
        impl Cast<bool> for $T {
            #[inline] fn cast(self) -> bool {
                let value = Cast::<i32>::cast(self);
                if  value == 0_i32 {
                    false
                } else if value == 1 {
                    true
                } else {
                    panic!("can not cast {value:?} to bool")
                }
            }
        }

        #[cfg(feature="time")]
        impl Cast<DateTime> for $T {
            #[inline] fn cast(self) -> DateTime { Cast::<i64>::cast(self).into() }
        }
        #[cfg(feature="time")]
        impl Cast<TimeDelta> for $T {
            #[inline] fn cast(self) -> TimeDelta { Cast::<i64>::cast(self).into() }
        }
    };

    ($T: ty => { $( $U: ty ),* } ) => {
        impl_numeric_cast!(@common_impl $T => { $( $U ),* });
        impl_numeric_cast!(@ $T => { $( $U),* });
        // impl_numeric_cast!(@ $T => { u8, u16, u32, u64, usize});
        // impl_numeric_cast!(@ $T => { i8, i16, i32, i64, isize });
    };

    (nocommon $T: ty => { $( $U: ty ),* } ) => {
        impl_numeric_cast!(@ $T => { $( $U),* });
    };
}

impl_numeric_cast!(u64 => { f32, f64, i32, i64, usize, isize });
impl_numeric_cast!(i64 => { f32, f64, i32, u64, usize, isize });
impl_numeric_cast!(i32 => { f32, f64, i64, u64, usize, isize });
impl_numeric_cast!(f32 => { f64, i32, i64, u64, usize, isize  });
impl_numeric_cast!(f64 => { f32, i32, i64, u64, usize, isize  });
impl_numeric_cast!(usize => { f32, f64, i32, i64, u64, isize });
impl_numeric_cast!(isize => { f32, f64, i32, i64, u64, usize });
// impl_numeric_cast!(char => { char });
impl_numeric_cast!(nocommon bool => {i32, i64, usize, isize});

impl Cast<String> for bool {
    #[inline]
    fn cast(self) -> String {
        self.to_string()
    }
}

macro_rules! impl_bool_cast {
    ($($T: ty),*) => {
        $(
            impl Cast<$T> for bool {
                #[inline] fn cast(self) -> $T { Cast::<i32>::cast(self).cast() }
            }
        )*
    };
}

#[cfg(feature = "time")]
macro_rules! impl_time_cast {
    ($($T: ty),*) => {
        $(
            impl Cast<$T> for DateTime {
                #[inline] fn cast(self) -> $T { Cast::<i64>::cast(self).cast() }
            }


            impl Cast<$T> for TimeDelta {
                #[inline] fn cast(self) -> $T { Cast::<i64>::cast(self).cast() }
            }
        )*

    };
}

#[cfg(feature = "time")]
impl Cast<i64> for DateTime {
    #[inline(always)]
    fn cast(self) -> i64 {
        self.into_i64()
    }
}

#[cfg(feature = "time")]
impl Cast<i64> for TimeDelta {
    #[inline]
    fn cast(self) -> i64 {
        let months = self.months;
        if months != 0 {
            panic!("not support cast TimeDelta to i64 when months is not zero")
        } else {
            self.inner.num_microseconds().unwrap_or(i64::MIN)
        }
    }
}

impl_bool_cast!(f32, f64);
#[cfg(feature = "time")]
impl_time_cast!(f32, f64, i32, u64, usize, isize, bool);

macro_rules! impl_cast_from_string {
    ($($T: ty),*) => {
        $(
            impl Cast<$T> for String {
                #[inline] fn cast(self) -> $T { self.parse().expect("Parse string error") }
            }

            impl Cast<$T> for &str {
                #[inline] fn cast(self) -> $T { self.parse().expect("Parse string error") }
            }
        )*
    };
}

impl_cast_from_string!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, char, bool);

impl Cast<String> for &str {
    #[inline]
    fn cast(self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "time")]
impl Cast<String> for DateTime {
    fn cast(self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "time")]
const TIME_RULE_VEC: [&str; 9] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M:%S.%f",
    "%Y-%m-%d",
    "%Y%m%d",
    "%Y%m%d %H%M%S",
    "%d/%m/%Y",
    "%d/%m/%Y H%M%S",
    "%Y%m%d%H%M%S",
    "%d/%m/%YH%M%S",
];

#[cfg(feature = "time")]
impl Cast<DateTime> for String {
    #[inline]
    fn cast(self) -> DateTime {
        for rule in TIME_RULE_VEC {
            if let Ok(dt) = DateTime::parse(&self, rule) {
                return dt;
            }
        }
        panic!("can not parse datetime from string: {self}")
    }
}

#[cfg(feature = "time")]
impl Cast<DateTime> for &str {
    #[inline]
    fn cast(self) -> DateTime {
        for rule in TIME_RULE_VEC {
            if let Ok(dt) = DateTime::parse(self, rule) {
                return dt;
            }
        }
        panic!("can not parse datetime from string: {self}")
    }
}

#[cfg(feature = "time")]
impl Cast<String> for TimeDelta {
    #[inline]
    fn cast(self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for DateTime {
    #[inline(always)]
    fn cast(self) -> TimeDelta {
        unreachable!()
    }
}

#[cfg(feature = "time")]
impl Cast<DateTime> for TimeDelta {
    #[inline(always)]
    fn cast(self) -> DateTime {
        unreachable!()
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for &str {
    #[inline(always)]
    fn cast(self) -> TimeDelta {
        TimeDelta::parse(self)
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for String {
    #[inline(always)]
    fn cast(self) -> TimeDelta {
        TimeDelta::parse(&self)
    }
}

pub trait IterCast: IntoIterator {
    #[inline]
    fn cast_iter<T>(self) -> impl Iterator<Item = T>
    where
        Self: Sized,
        Self::Item: Cast<T>,
    {
        self.into_iter().map(|item| item.cast())
    }
}

pub trait OptIterCast<T>: IntoIterator<Item = Option<T>> {
    #[inline]
    fn cast_opt_iter<U>(self) -> impl Iterator<Item = Option<U>>
    where
        Self: Sized,
        T: Cast<U>,
    {
        self.into_iter().map(|item| item.map(|v| v.cast()))
    }
}

impl<I: IntoIterator> IterCast for I {}
impl<I: IntoIterator<Item = Option<T>>, T> OptIterCast<T> for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iter_cast() {
        let v = vec![1, 2, 3, 4, 5];
        let v: Vec<f32> = v.into_iter().cast_iter().collect();
        assert_eq!(v, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_opt_iter_cast() {
        let v = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
        let v: Vec<Option<f32>> = v.into_iter().cast_opt_iter().collect();
        assert_eq!(
            v,
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)]
        );
    }
}
