#![allow(unreachable_patterns)]
use crate::prelude::*;
use tea_macros::GetDtype;

#[derive(GetDtype, Debug, Clone)]
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

impl Scalar {
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn titer(&self) -> TResult<DynTrustIter> {
        if let Scalar::VecUsize(v) = self {
            // clone vector is expensive, so we use reference instead
            Ok(v.titer().into())
        } else {
            self.clone().into_titer()
        }
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn into_titer(self) -> TResult<DynTrustIter<'static>> {
        match_scalar!(self; dynamic(v) => Ok(std::iter::once(v).into()),)
    }

    #[inline]
    pub fn cheap_clone(&self) -> Option<Self> {
        match_scalar!(
            self;
            numeric(v) => Ok((*v).into()),
            U8(v) => Ok((*v).into()),
            Bool(v) => Ok((*v).into()),
            String(v) => Ok(v.clone().into()),
            #[cfg(feature = "time")] DateTime(v) => Ok((*v).into()),
            #[cfg(feature = "time")] TimeDelta(v) => Ok((*v).into()),
        )
        .ok()
    }
    #[inline]
    pub fn cast_i32(self) -> TResult<i32> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }

    #[inline]
    pub fn cast_i64(self) -> TResult<i64> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }

    #[inline]
    pub fn cast_f32(self) -> TResult<f32> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }

    #[inline]
    pub fn cast_f64(self) -> TResult<f64> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }

    #[inline]
    pub fn cast_bool(self) -> TResult<bool> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }

    #[inline]
    pub fn cast_usize(self) -> TResult<usize> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }

    #[inline]
    pub fn cast_optusize(self) -> TResult<Option<usize>> {
        match_scalar!(self; numeric(v) => Ok(v.cast()),)
    }
}

macro_rules! impl_cast {
    ($($(#[$meta: meta])? $real: ty),*) => {
        $(
            $(#[$meta])?
            impl Cast<$real> for Scalar {
                #[inline]
                fn cast(self) -> $real {
                    match_scalar!(self; cast(v) => Ok(v.cast()),).unwrap()
                }
            }
        )*
    };
}

impl_cast!(
    bool,
    f32,
    f64,
    i32,
    i64,
    u8,
    u64,
    usize,
    String,
    Option<usize>,
    #[cfg(feature = "time")]
    DateTime,
    #[cfg(feature = "time")]
    TimeDelta
);

impl Cast<Vec<usize>> for Scalar {
    #[inline]
    #[allow(unreachable_patterns)]
    fn cast(self) -> Vec<usize> {
        match_scalar!(
            self;
            cast(v) => {Ok(vec![v.cast()])},
            VecUsize(v) => Ok(v.cast()),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_dtype() {
        let s = Scalar::F64(1.0);
        assert_eq!(s.dtype(), DataType::F64);
    }
}
