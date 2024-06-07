use crate::{match_enum, prelude::*};

// trait ScalarElement {}

// impl ScalarElement for bool {}
// impl ScalarElement for f32 {}
// impl ScalarElement for f64 {}
// impl ScalarElement for i32 {}
// impl ScalarElement for i64 {}
// impl ScalarElement for u8 {}
// impl ScalarElement for u64 {}
// impl ScalarElement for usize {}
// impl ScalarElement for String {}
// impl ScalarElement for Option<usize> {}
// #[cfg(feature = "time")]
// impl ScalarElement for DateTime {}
// #[cfg(feature = "time")]
// impl ScalarElement for TimeDelta {}

// impl<T: ScalarElement + Sized> TransmuteDtype for T {
//     type Output<U: Sized> = U;

//     #[inline]
//     /// # Safety
//     ///
//     /// the caller must ensure T and U is actually the same type
//     unsafe fn into_dtype<U: Sized>(self) -> Self::Output<U> {
//         std::mem::transmute(self)
//     }
// }

#[derive(Debug, Clone)]
pub enum Scalar {
    Bool(bool),
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    U8(u8),
    U64(u64),
    Usize(usize),
    String(String),
    OptUsize(Option<usize>),
    VecUsize(Vec<usize>),
    #[cfg(feature = "time")]
    DateTime(DateTime),
    #[cfg(feature = "time")]
    TimeDelta(TimeDelta),
}

macro_rules! impl_from {

    ($($(#[$meta:meta])? ($arm: ident, $ty: ty, $func_name: ident)),* $(,)?) => {
        impl Scalar {
            $(
                $(#[$meta])?
                pub fn $func_name(self) -> TResult<$ty> {
                    if let Scalar::$arm(v) = self {
                        Ok(v)
                    } else {
                        tbail!("Scalar is not of type {:?}", DataType::$arm)
                    }
            })*
        }

        // $(
        //     $(#[$meta])?
        //     impl Cast<$ty> for Scalar {
        //         #[inline]
        //         fn cast(self) -> $ty {
        //             $crate::match_scalar!(numeric self, v, {v.cast()}).unwrap()
        //         }
        //     }
        // )*

        impl<T: GetDataType> From<T> for Scalar {
            #[allow(unreachable_patterns)]
            #[inline]
            fn from(v: T) -> Self {
                match T::dtype() {
                    $(
                        $(#[$meta])? DataType::$arm => {
                            // safety: we have checked the type
                            let v: $ty = unsafe{std::mem::transmute_copy(&v)};
                            Scalar::$arm(v.into())
                        },
                    )*
                    type_ => unimplemented!("Create Scalar from type {:?} is not implemented", type_),
                }
            }
        }
    };
}

impl_from!(
    (Bool, bool, bool),
    (F32, f32, f32),
    (F64, f64, f64),
    (I32, i32, i32),
    (I64, i64, i64),
    (U8, u8, u8),
    (U64, u64, u64),
    (Usize, usize, usize),
    (String, String, string),
    (OptUsize, Option<usize>, opt_usize),
    (VecUsize, Vec<usize>, vec_usize),
    #[cfg(feature = "time")]
    (DateTime, DateTime, datetime),
    #[cfg(feature = "time")]
    (TimeDelta, TimeDelta, timedelta)
);

#[macro_export]
macro_rules! match_scalar {
    ($($tt: tt)*) => {
        $crate::match_enum!(Scalar, $($tt)*)
    };
}

impl Cast<f64> for Scalar {
    #[inline]
    fn cast(self) -> f64 {
        crate::match_enum!(
            Scalar,
            self,
            F32(v) | F64(v) => v as f64
        )
        .unwrap()
    }
}

impl Scalar {
    #[inline]
    #[allow(unreachable_patterns, clippy::should_implement_trait)]
    pub fn to_iter(&self) -> TResult<DynTrustIter> {
        if let Scalar::VecUsize(v) = self {
            // clone vector is expensive, so we use reference instead
            Ok(v.to_iter().into())
        } else {
            self.clone().into_iter()
        }
    }

    #[inline]
    #[allow(unreachable_patterns, clippy::should_implement_trait)]
    pub fn into_iter(self) -> TResult<DynTrustIter<'static>> {
        match_scalar!(dynamic self, v, {std::iter::once(v).into()})
    }

    #[inline]
    pub fn cast_i32(self) -> TResult<i32> {
        match_scalar!(numeric self, v, {v.cast()})
    }

    #[inline]
    pub fn cast_i64(self) -> TResult<i64> {
        match_scalar!(numeric self, v, {v.cast()})
    }

    #[inline]
    pub fn cast_f32(self) -> TResult<f32> {
        match_scalar!(numeric self, v, {v.cast()})
    }

    #[inline]
    pub fn cast_f64(self) -> TResult<f64> {
        match_scalar!(numeric self, v, {v.cast()})
    }

    #[inline]
    pub fn cast_bool(self) -> TResult<bool> {
        match_scalar!(numeric self, v, {v.cast()})
    }

    #[inline]
    pub fn cast_usize(self) -> TResult<usize> {
        match_scalar!(numeric self, v, {v.cast()})
    }

    #[inline]
    pub fn cast_optusize(self) -> TResult<Option<usize>> {
        match_scalar!(numeric self, v, {v.cast()})
    }
}
