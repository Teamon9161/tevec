#[cfg(feature = "time")]
use tea_time::{DateTime, TimeDelta};

use super::isnone::IsNone;

pub trait Cast<T> {
    fn cast(self) -> T;
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
    (@ $T: ty => impl $U: ty ) => {
        impl Cast<$U> for $T {
            #[inline] fn cast(self) -> $U { self as $U }
        }

        impl Cast<Option<$U>> for $T {
            #[inline] fn cast(self) -> Option<$U> {
                if self.is_none() {
                    None
                } else {
                    Some(self as $U)
                }
            }
        }

        impl Cast<$U> for Option<$T> {
            #[inline] fn cast(self) -> $U { self.map(|v| v as $U).unwrap_or_else(<$U as IsNone>::none)}
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


        impl Cast<bool> for Option<$T> {
            #[inline] fn cast(self) -> bool {
                self.expect("can not cast None to bool").cast()
            }
        }

        impl Cast<String> for Option<$T> {
            #[inline] fn cast(self) -> String {
                self.map(|v| v.to_string()).unwrap_or("None".to_string())
            }
        }

        #[cfg(feature="time")]
        impl Cast<DateTime> for Option<$T> {
            #[inline] fn cast(self) -> DateTime { self.map(|v| v.cast()).unwrap_or(DateTime(None)) }
        }

        #[cfg(feature="time")]
        impl Cast<TimeDelta> for Option<$T> {
            #[inline] fn cast(self) -> TimeDelta { self.map(|v| v.cast()).unwrap_or(TimeDelta::nat()) }
        }
    };

    ($T: ty => { $( $U: ty ),* } ) => {
        impl Cast<$T> for Option<$T> {
            #[inline(always)]
            fn cast(self) -> $T {
                self.unwrap_or_else(<$T as IsNone>::none)
            }
        }
        impl_numeric_cast!(@common_impl $T => { $( $U ),* });
        impl_numeric_cast!(@ $T => { $( $U),* });
    };

    (nocommon $T: ty => { $( $U: ty ),* } ) => {
        impl_numeric_cast!(@ $T => { $( $U),* });
    };
}

impl_numeric_cast!(u8 => { u64, f32, f64, i32, i64, usize, isize });
impl_numeric_cast!(u64 => { u8, f32, f64, i32, i64, usize, isize });
impl_numeric_cast!(i64 => { u8, f32, f64, i32, u64, usize, isize });
impl_numeric_cast!(i32 => { u8, f32, f64, i64, u64, usize, isize });
impl_numeric_cast!(f32 => { u8, f64, i32, i64, u64, usize, isize  });
impl_numeric_cast!(f64 => { u8, f32, i32, i64, u64, usize, isize  });
impl_numeric_cast!(usize => { u8, f32, f64, i32, i64, u64, isize });
impl_numeric_cast!(isize => { u8, f32, f64, i32, i64, u64, usize });
// impl_numeric_cast!(char => { char });
impl_numeric_cast!(nocommon bool => {u8, i32, i64, u64, usize, isize});

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

            impl Cast<Option<$T>> for String {
                #[inline]
                fn cast(self) -> Option<$T> {
                    if self == "None" {
                        None
                    } else {
                        Some(self.cast())
                    }
                }
            }

            impl Cast<Option<$T>> for &str {
                #[inline]
                fn cast(self) -> Option<$T> {
                    if self == "None" {
                        None
                    } else {
                        Some(self.cast())
                    }
                }
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
impl Cast<DateTime> for String {
    #[inline]
    fn cast(self) -> DateTime {
        self.parse().expect("Parse string to datetime error")
    }
}

#[cfg(feature = "time")]
impl Cast<DateTime> for &str {
    #[inline]
    fn cast(self) -> DateTime {
        self.parse().expect("Parse str to datetime error")
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
        TimeDelta::parse(self).expect("Parse str to timedelta error")
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for String {
    #[inline(always)]
    fn cast(self) -> TimeDelta {
        TimeDelta::parse(&self).expect("Parse string to timedelta error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_option_cast() {
        let a = Some(1_usize);
        let a: i32 = a.cast();
        assert_eq!(a, 1);

        let b: Option<usize> = None;
        let b: f64 = b.cast();
        assert!(b.is_nan());

        let c: Option<usize> = Some(3);
        let c: usize = c.cast();
        assert_eq!(c, 3);

        let d = Some(1usize);
        let d: bool = d.cast();
        assert!(d);
    }
}
