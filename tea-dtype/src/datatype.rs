#[cfg(feature = "time")]
use tea_time::TimeUnitTrait;
#[cfg(feature = "time")]
use tea_time::{DateTime, TimeDelta, TimeUnit};

#[derive(PartialEq, Eq, Debug)]
pub enum DataType {
    Bool,
    F32,
    F64,
    I32,
    I64,
    U8,
    U64,
    Usize,
    Str,
    String,
    Object,
    OptBool,
    OptF32,
    OptF64,
    OptI32,
    OptI64,
    OptUsize,
    VecUsize,
    #[cfg(feature = "time")]
    DateTime(TimeUnit),
    #[cfg(feature = "time")]
    TimeDelta,
}

pub trait GetDataType {
    fn dtype() -> DataType
    where
        Self: Sized;
}

macro_rules! impl_datatype {
    ($tyname:ident, $physical:ty) => {
        impl GetDataType for $physical {
            #[inline(always)]
            fn dtype() -> DataType {
                DataType::$tyname
            }
        }
    };
}

impl DataType {
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        use DataType::*;
        matches!(self, F32 | F64 | OptF32 | OptF64)
    }

    #[inline(always)]
    pub fn is_int(&self) -> bool {
        use DataType::*;
        matches!(self, I32 | I64 | Usize | U64 | OptUsize | OptI32 | OptI64)
    }

    pub fn float(self) -> Self {
        use DataType::*;
        match self {
            F32 => F32,
            I32 => F32,
            I64 => F64,
            Usize => F64,
            U64 => F64,
            OptUsize => F64,
            OptI32 => OptF32,
            OptI64 => OptF64,
            OptF32 => OptF32,
            OptF64 => OptF64,
            _ => F64,
        }
    }

    pub fn int(self) -> Self {
        use DataType::*;
        match self {
            I32 => I32,
            F32 => I32,
            F64 => I64,
            Usize => Usize,
            U64 => I64,
            OptUsize => OptUsize,
            OptI32 => OptI32,
            OptI64 => OptI64,
            OptF32 => OptI32,
            OptF64 => OptI64,
            _ => I64,
        }
    }
}

impl_datatype!(Bool, bool);
impl_datatype!(U8, u8);
impl_datatype!(F32, f32);
impl_datatype!(F64, f64);
impl_datatype!(I32, i32);
impl_datatype!(I64, i64);
impl_datatype!(U64, u64);
impl_datatype!(Usize, usize);
impl_datatype!(String, String);
impl_datatype!(OptBool, Option<bool>);
impl_datatype!(OptF32, Option<f32>);
impl_datatype!(OptF64, Option<f64>);
impl_datatype!(OptI32, Option<i32>);
impl_datatype!(OptI64, Option<i64>);
impl_datatype!(OptUsize, Option<usize>);
impl_datatype!(VecUsize, Vec<usize>);

#[cfg(feature = "time")]
impl<U: TimeUnitTrait> GetDataType for DateTime<U> {
    #[inline(always)]
    fn dtype() -> DataType {
        DataType::DateTime(U::unit())
    }
}

#[cfg(feature = "time")]
impl_datatype!(TimeDelta, TimeDelta);

impl<'a> GetDataType for &'a str {
    // type Physical = &'a str;
    #[inline(always)]
    fn dtype() -> DataType {
        DataType::Str
    }
}
