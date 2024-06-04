// use tea_dtype::GetDataType;
use crate::prelude::*;
// use crate::prelude::TrustedLen;

pub trait TransmuteDtype {
    type Output<U>;
    unsafe fn into_dtype<T>(self) -> Self::Output<T>;
}

impl<T> TransmuteDtype for Box<dyn TrustedLen<Item = T>> {
    type Output<U> = Box<dyn TrustedLen<Item = U>>;

    unsafe fn into_dtype<U>(self) -> Self::Output<U> {
        std::mem::transmute(self)
    }
}

pub enum DynTrustIter {
    Bool(Box<dyn TrustedLen<Item = bool>>),
    F32(Box<dyn TrustedLen<Item = f32>>),
    F64(Box<dyn TrustedLen<Item = f64>>),
    I32(Box<dyn TrustedLen<Item = i32>>),
    I64(Box<dyn TrustedLen<Item = i64>>),
    U8(Box<dyn TrustedLen<Item = u8>>),
    U64(Box<dyn TrustedLen<Item = u64>>),
    Usize(Box<dyn TrustedLen<Item = usize>>),
    String(Box<dyn TrustedLen<Item = String>>),
    OptUsize(Box<dyn TrustedLen<Item = Option<usize>>>),
    VecUsize(Box<dyn TrustedLen<Item = Vec<usize>>>),
    #[cfg(feature = "time")]
    DateTime(Box<dyn TrustedLen<Item = DateTime>>),
    #[cfg(feature = "time")]
    TimeDelta(Box<dyn TrustedLen<Item = TimeDelta>>),
}

impl DynTrustIter {
    pub fn dtype(&self) -> DataType {
        match self {
            DynTrustIter::Bool(_) => DataType::Bool,
            DynTrustIter::F32(_) => DataType::F32,
            DynTrustIter::F64(_) => DataType::F64,
            DynTrustIter::I32(_) => DataType::I32,
            DynTrustIter::I64(_) => DataType::I64,
            DynTrustIter::U8(_) => DataType::U8,
            DynTrustIter::U64(_) => DataType::U64,
            DynTrustIter::Usize(_) => DataType::Usize,
            DynTrustIter::String(_) => DataType::String,
            DynTrustIter::OptUsize(_) => DataType::OptUsize,
            DynTrustIter::VecUsize(_) => DataType::VecUsize,
            #[cfg(feature = "time")]
            DynTrustIter::DateTime(_) => DataType::DateTime,
            #[cfg(feature = "time")]
            DynTrustIter::TimeDelta(_) => DataType::TimeDelta,
        }
    }

    pub fn cast_to(self, dtype: DataType) -> TResult<DynTrustIter> {
        let res: DynTrustIter = match dtype {
            DataType::Bool => self.bool()?.into(),
            DataType::F32 => self.f32()?.into(),
            DataType::F64 => self.f64()?.into(),
            DataType::I32 => self.i32()?.into(),
            DataType::I64 => self.i64()?.into(),
            DataType::U8 => self.u8()?.into(),
            DataType::U64 => self.u64()?.into(),
            DataType::Usize => self.usize()?.into(),
            DataType::String => self.string()?.into(),
            DataType::OptUsize => self.opt_usize()?.into(),
            DataType::VecUsize => self.vec_usize()?.into(),
            #[cfg(feature = "time")]
            DataType::DateTime => self.datetime()?.into(),
            #[cfg(feature = "time")]
            DataType::TimeDelta => self.timedelta()?.into(),
            _ => tbail!("Cast to type {:?} for TrustIter is not implemented", dtype),
        };
        Ok(res)
    }
}

macro_rules! impl_from {

    ($($(#[$meta:meta])? ($arm: ident, $ty: ty, $func_name: ident)),* $(,)?) => {
        impl DynTrustIter {
            $(
                $(#[$meta])?
                pub fn $func_name(self) -> TResult<Box<dyn TrustedLen<Item = $ty>>> {
                    if let DynTrustIter::$arm(v) = self {
                        Ok(v)
                    } else {
                        tbail!("TrustIter is not of type {:?}", DataType::$arm)
                    }
            })*
        }

        impl<T: GetDataType, I: TrustedLen<Item=T> + 'static> From<I> for DynTrustIter {
            #[allow(unreachable_patterns)]
            fn from(iter: I) -> Self {
                match T::dtype() {
                    $(
                        $(#[$meta])? DataType::$arm => {
                            let iter: Box<dyn TrustedLen<Item=T>> = Box::new(iter);
                            // safety: we have checked the type
                            unsafe{DynTrustIter::$arm(iter.into_dtype::<$ty>().into())}
                        },
                    )*
                    type_ => unimplemented!("Create TrustIter from type {:?} is not implemented", type_),
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
