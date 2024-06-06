use super::TransmuteDtype;
use crate::prelude::*;

impl<T> TransmuteDtype for Vec<T> {
    type Output<U> = Vec<U>;

    #[inline]
    /// # Safety
    ///
    /// the caller must ensure T and U is actually the same type
    unsafe fn into_dtype<U>(self) -> Self::Output<U> {
        std::mem::transmute(self)
    }
}

#[derive(Debug, Clone)]
pub enum DynVec {
    Bool(Vec<bool>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    U8(Vec<u8>),
    U64(Vec<u64>),
    Usize(Vec<usize>),
    String(Vec<String>),
    OptUsize(Vec<Option<usize>>),
    VecUsize(Vec<Vec<usize>>),
    #[cfg(feature = "time")]
    DateTime(Vec<DateTime>),
    #[cfg(feature = "time")]
    TimeDelta(Vec<TimeDelta>),
}

macro_rules! impl_from {

    ($($(#[$meta:meta])? ($arm: ident, $ty: ty, $func_name: ident)),* $(,)?) => {
        impl DynVec {
            $(
                $(#[$meta])?
                pub fn $func_name(self) -> TResult<Vec<$ty>> {
                    if let DynVec::$arm(v) = self {
                        Ok(v)
                    } else {
                        tbail!("Vector is not of type {:?}", DataType::$arm)
                    }
            })*
        }

        impl<T: GetDataType> From<Vec<T>> for DynVec {
            #[allow(unreachable_patterns)]
            #[inline]
            fn from(vec: Vec<T>) -> Self {
                match T::dtype() {
                    $(
                        $(#[$meta])? DataType::$arm => {
                            // safety: we have checked the type
                            unsafe{DynVec::$arm(vec.into_dtype::<$ty>().into())}
                        },
                    )*
                    type_ => unimplemented!("Create Vector from type {:?} is not implemented", type_),
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
macro_rules! match_vec {
    ($($tt: tt)*) => {
        $crate::match_enum!(DynVec, $($tt)*)
    };
}

#[macro_export]
/// create dynamic iter
macro_rules! d_vec {
    ($($tt: tt)*) => {
        {
            let vec: DynVec = vec![ $($tt)* ].into();
            vec
        }
    };
}

impl DynVec {
    #[inline]
    #[allow(unreachable_patterns)]
    pub fn to_iter(&self) -> DynTrustIter {
        match_vec!(dynamic self, v, { v.to_iter().into() })
    }

    #[inline]
    #[allow(unreachable_patterns, clippy::should_implement_trait)]
    pub fn into_iter(self) -> DynTrustIter<'static> {
        match_vec!(dynamic self, v, { v.into_iter().into() })
    }
}
