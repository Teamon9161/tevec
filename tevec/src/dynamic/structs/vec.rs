#![allow(unreachable_patterns)]
use super::TransmuteDtype;
use crate::prelude::*;
use tea_macros::GetDtype;

impl<T, U> TransmuteDtype<U> for Vec<T> {
    type Output = Vec<U>;

    #[inline]
    /// # Safety
    ///
    /// the caller must ensure T and U is actually the same type
    unsafe fn into_dtype(self) -> Self::Output {
        std::mem::transmute(self)
    }
}

#[derive(GetDtype, Debug, Clone)]
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
                            unsafe{DynVec::$arm(vec.into_dtype().into())}
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
    pub fn len(&self) -> usize {
        match_vec!(self; dynamic(v) => Ok(v.len()),).unwrap()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get(&self, index: usize) -> TResult<Scalar> {
        match_vec!(self; dynamic(v) => v.get(index).map(|v| v.into()),)
    }

    #[inline]
    pub fn titer(&self) -> TResult<DynTrustIter> {
        match_vec!(self; dynamic(v) => Ok(v.titer().into()),)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn into_titer(self) -> TResult<DynTrustIter<'static>> {
        match_vec!(self; dynamic(v) => Ok(v.into_iter().into()),)
    }
}
