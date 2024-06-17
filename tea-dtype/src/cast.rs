use crate::*;

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

        impl Cast<Option<$U>> for Option<$T> {
            #[inline] fn cast(self) -> Option<$U> {
                self.map(|v| v.cast())
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

        // cast for bool type
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

        impl Cast<Option<bool>> for $T {
            #[inline] fn cast(self) -> Option<bool> {
                if self.is_none() {
                    None
                } else {
                    Some(self.cast())
                }
            }
        }

        impl Cast<bool> for Option<$T> {
            #[inline] fn cast(self) -> bool {
                self.expect("can not cast None to bool").cast()
            }
        }

        impl Cast<Option<bool>> for Option<$T> {
            #[inline] fn cast(self) -> Option<bool> {
                self.map(|v| v.cast())
            }
        }

        impl Cast<$T> for bool {
            #[inline] fn cast(self) -> $T { (self as u8).cast() }
        }

        impl Cast<Option<$T>> for bool {
            #[inline] fn cast(self) -> Option<$T> { Some((self as u8).cast()) }
        }

        impl Cast<$T> for Option<bool> {
            #[inline] fn cast(self) -> $T {
                if let Some(v) = self {
                    v.cast()
                } else {
                    <$T as IsNone>::none()
                }
             }
        }

        impl Cast<Option<$T>> for Option<bool> {
            #[inline] fn cast(self) -> Option<$T> { self.map(Cast::cast) }
        }

        // cast for string type
        impl Cast<String> for $T {
            #[inline] fn cast(self) -> String { self.to_string() }
        }

        impl Cast<String> for Option<$T> {
            #[inline] fn cast(self) -> String {
                self.map(|v| v.to_string()).unwrap_or("None".to_string())
            }
        }

        #[cfg(feature="time")]
        impl<U: TimeUnitTrait> Cast<DateTime<U>> for $T {
            #[inline] fn cast(self) -> DateTime<U> { Cast::<i64>::cast(self).into() }
        }

        #[cfg(feature="time")]
        impl Cast<TimeDelta> for $T {
            #[inline] fn cast(self) -> TimeDelta { Cast::<i64>::cast(self).into() }
        }

        #[cfg(feature="time")]
        impl<U: TimeUnitTrait> Cast<DateTime<U>> for Option<$T> {
            #[inline] fn cast(self) -> DateTime<U> { self.map(|v| v.cast()).unwrap_or(DateTime::nat()) }
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

impl Cast<String> for bool {
    #[inline]
    fn cast(self) -> String {
        self.to_string()
    }
}

impl Cast<bool> for Option<bool> {
    #[inline]
    fn cast(self) -> bool {
        if let Some(v) = self {
            v
        } else {
            panic!("Should not cast None to bool")
        }
    }
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<DateTime<U>> for bool {
    #[inline]
    fn cast(self) -> DateTime<U> {
        panic!("Should not cast bool to datetime")
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for bool {
    #[inline]
    fn cast(self) -> TimeDelta {
        panic!("Should not cast bool to timedelta")
    }
}

impl Cast<String> for Option<bool> {
    #[inline]
    fn cast(self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<DateTime<U>> for Option<bool> {
    #[inline]
    fn cast(self) -> DateTime<U> {
        panic!("Should not cast option bool to datetime")
    }
}

#[cfg(feature = "time")]
impl Cast<TimeDelta> for Option<bool> {
    #[inline]
    fn cast(self) -> TimeDelta {
        panic!("Should not cast option bool to timedelta")
    }
}

#[cfg(feature = "time")]
macro_rules! impl_time_cast {
    ($($T: ty),*) => {
        $(
            impl<U: TimeUnitTrait> Cast<$T> for DateTime<U> {
                #[inline] fn cast(self) -> $T { Cast::<i64>::cast(self).cast() }
            }

            impl<U: TimeUnitTrait> Cast<Option<$T>> for DateTime<U> {
                #[inline] fn cast(self) -> Option<$T> {
                    if self.is_none() {
                        None
                    } else {
                        Some(self.cast())
                    }
                 }
            }


            impl Cast<$T> for TimeDelta {
                #[inline] fn cast(self) -> $T { Cast::<i64>::cast(self).cast() }
            }

            impl Cast<Option<$T>> for TimeDelta {
                #[inline] fn cast(self) -> Option<$T> {
                    if self.is_none() {
                        None
                    } else {
                        Some(self.cast())
                    }
                 }
            }
        )*
    };
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<i64> for DateTime<U> {
    #[inline(always)]
    fn cast(self) -> i64 {
        self.into_i64()
    }
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<Option<i64>> for DateTime<U> {
    #[inline(always)]
    fn cast(self) -> Option<i64> {
        self.into_opt_i64()
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

#[cfg(feature = "time")]
impl Cast<Option<i64>> for TimeDelta {
    #[inline]
    fn cast(self) -> Option<i64> {
        let months = self.months;
        if months != 0 {
            panic!("not support cast TimeDelta to i64 when months is not zero")
        } else {
            self.inner.num_microseconds().map(Some).unwrap_or(None)
        }
    }
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
// impl_numeric_cast!(nocommon bool => {u8, i32, i64, u64, usize, isize});

#[cfg(feature = "time")]
impl_time_cast!(u8, u64, f32, f64, i32, usize, isize, bool);

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
impl<U: TimeUnitTrait> Cast<String> for DateTime<U> {
    fn cast(self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<DateTime<U>> for String
where
    DateTime<U>: From<CrDateTime<Utc>>,
{
    #[inline]
    fn cast(self) -> DateTime<U> {
        self.parse().expect("Parse string to datetime error")
    }
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<DateTime<U>> for &str
where
    DateTime<U>: From<CrDateTime<Utc>>,
{
    #[inline]
    fn cast(self) -> DateTime<U> {
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
impl<U: TimeUnitTrait> Cast<TimeDelta> for DateTime<U> {
    #[inline(always)]
    fn cast(self) -> TimeDelta {
        unreachable!()
    }
}

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> Cast<DateTime<U>> for TimeDelta {
    #[inline(always)]
    fn cast(self) -> DateTime<U> {
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

#[cfg(feature = "time")]
// TODO: maybe we can remove default Cast<Self>?
// we can impl Cast<U: TimeUnitTrait> for DateTime<U> once we
// remove default implemention for Cast<Self>
macro_rules! time_unit_cast {
    ($($unit: ident => ($($to_unit: ident),*)),*) => {
        $($(impl Cast<DateTime<unit::$to_unit>> for DateTime<unit::$unit> {
            #[inline(always)]
            fn cast(self) -> DateTime<unit::$to_unit> {
                self.into_unit::<unit::$to_unit>()
            }
        })*)*
    // ($($unit: ident),*) => {
    //     $(impl<U: TimeUnitTrait> Cast<DateTime<U>> for DateTime<unit::$unit> {
    //         #[inline(always)]
    //         fn cast(self) -> DateTime<U> {
    //             self.into_unit::<U>()
    //         }
    //     })*
    };
}

#[cfg(feature = "time")]
// time_unit_cast!(Millisecond, Microsecond, Second, Nanosecond);
time_unit_cast!(
    Millisecond => (Microsecond, Second, Nanosecond),
    Microsecond => (Millisecond, Second, Nanosecond),
    Second => (Millisecond, Microsecond, Nanosecond),
    Nanosecond => (Millisecond, Microsecond, Second)
);

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

        let e = Some(2i32);
        let e: Option<f64> = e.cast();
        assert_eq!(e, Some(2.0));
    }
}
