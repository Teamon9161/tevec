#[cfg(feature = "time")]
use tea_time::TimeUnitTrait;
#[cfg(feature = "time")]
use tea_time::{DateTime, TimeDelta, TimeUnit};
/// Represents the various data types supported by the system.
///
/// This enum encapsulates both primitive and complex data types, including
/// numeric types, strings, optional types, and time-related types (when the "time" feature is enabled).
#[derive(PartialEq, Eq, Debug)]
pub enum DataType {
    /// Boolean type
    Bool,
    /// 32-bit floating point type
    F32,
    /// 64-bit floating point type
    F64,
    /// 32-bit signed integer type
    I32,
    /// 64-bit signed integer type
    I64,
    /// 8-bit unsigned integer type
    U8,
    /// 64-bit unsigned integer type
    U64,
    /// Platform-specific size type
    Usize,
    /// String slice type
    Str,
    /// Owned String type
    String,
    /// Generic Object type
    Object,
    /// Optional Boolean type
    OptBool,
    /// Optional 32-bit floating point type
    OptF32,
    /// Optional 64-bit floating point type
    OptF64,
    /// Optional 32-bit signed integer type
    OptI32,
    /// Optional 64-bit signed integer type
    OptI64,
    /// Optional platform-specific size type
    OptUsize,
    /// Vector of platform-specific size type
    VecUsize,
    /// DateTime type with specified TimeUnit (only available with "time" feature)
    #[cfg(feature = "time")]
    DateTime(TimeUnit),
    /// TimeDelta type (only available with "time" feature)
    #[cfg(feature = "time")]
    TimeDelta,
}

/// A trait for types that can provide their corresponding DataType.
pub trait GetDataType {
    /// Returns the DataType corresponding to the implementing type.
    ///
    /// # Returns
    ///
    /// A `DataType` enum variant representing the type.
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
    /// Checks if the DataType is a floating-point type.
    ///
    /// # Returns
    ///
    /// `true` if the DataType is F32, F64, OptF32, or OptF64, `false` otherwise.
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        use DataType::*;
        matches!(self, F32 | F64 | OptF32 | OptF64)
    }

    /// Checks if the DataType is an integer type.
    ///
    /// # Returns
    ///
    /// `true` if the DataType is I32, I64, Usize, U64, OptUsize, OptI32, or OptI64, `false` otherwise.
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        use DataType::*;
        matches!(self, I32 | I64 | Usize | U64 | OptUsize | OptI32 | OptI64)
    }

    /// Checks if the DataType is a time-related type.
    ///
    /// # Returns
    ///
    /// `true` if the DataType is DateTime (when the "time" feature is enabled), `false` otherwise.
    #[inline]
    pub fn is_time(&self) -> bool {
        #[cfg(feature = "time")]
        {
            use DataType::*;
            matches!(self, DateTime(_))
        }
        #[cfg(not(feature = "time"))]
        {
            false
        }
    }

    /// Converts the DataType to its floating-point equivalent.
    ///
    /// # Returns
    ///
    /// A new DataType that represents the floating-point equivalent of the current type.
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

    /// Converts the DataType to its integer equivalent.
    ///
    /// # Returns
    ///
    /// A new DataType that represents the integer equivalent of the current type.
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
