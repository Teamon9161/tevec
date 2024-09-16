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
    /// Unknown type
    Unknown,
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

#[cfg(feature = "polars")]
const fn into_pl_dtype(dt: &DataType) -> tea_deps::polars::prelude::DataType {
    use tea_deps::polars::prelude::{DataType as PlDataType, UnknownKind};
    match dt {
        DataType::Bool => PlDataType::Boolean,
        DataType::F32 => PlDataType::Float32,
        DataType::F64 => PlDataType::Float64,
        DataType::I32 => PlDataType::Int32,
        DataType::I64 => PlDataType::Int64,
        DataType::U8 => PlDataType::UInt8,
        DataType::U64 => PlDataType::UInt64,
        DataType::Usize => PlDataType::UInt64,
        DataType::String => PlDataType::String,
        _ => PlDataType::Unknown(UnknownKind::Any),
    }
}

#[cfg(feature = "polars")]
const fn from_pl_dtype(dt: &tea_deps::polars::prelude::DataType) -> DataType {
    use tea_deps::polars::prelude::DataType as PlDataType;
    match dt {
        PlDataType::Boolean => DataType::Bool,
        PlDataType::Float32 => DataType::F32,
        PlDataType::Float64 => DataType::F64,
        PlDataType::Int32 => DataType::I32,
        PlDataType::Int64 => DataType::I64,
        PlDataType::UInt8 => DataType::U8,
        PlDataType::UInt64 => DataType::U64,
        PlDataType::String => DataType::String,
        _ => DataType::Unknown,
    }
}

#[cfg(feature = "polars")]
impl From<DataType> for tea_deps::polars::prelude::DataType {
    #[inline(always)]
    fn from(dt: DataType) -> Self {
        into_pl_dtype(&dt)
    }
}

#[cfg(feature = "polars")]
impl From<&DataType> for tea_deps::polars::prelude::DataType {
    #[inline(always)]
    fn from(dt: &DataType) -> Self {
        into_pl_dtype(dt)
    }
}

#[cfg(feature = "polars")]
impl From<tea_deps::polars::prelude::DataType> for DataType {
    #[inline(always)]
    fn from(dt: tea_deps::polars::prelude::DataType) -> Self {
        from_pl_dtype(&dt)
    }
}

#[cfg(feature = "polars")]
impl From<&tea_deps::polars::prelude::DataType> for DataType {
    #[inline(always)]
    fn from(dt: &tea_deps::polars::prelude::DataType) -> Self {
        from_pl_dtype(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "polars")]
    fn test_from_datatype() {
        use tea_deps::polars::prelude::DataType as PlDataType;
        let dt: PlDataType = DataType::Bool.into();
        assert_eq!(dt, PlDataType::Boolean);
        let dt: PlDataType = DataType::F32.into();
        assert_eq!(dt, PlDataType::Float32);
        let dt: PlDataType = DataType::F64.into();
        assert_eq!(dt, PlDataType::Float64);
        let dt: PlDataType = DataType::I32.into();
        assert_eq!(dt, PlDataType::Int32);
        let dt: PlDataType = DataType::I64.into();
        assert_eq!(dt, PlDataType::Int64);
        let dt: PlDataType = DataType::U8.into();
        assert_eq!(dt, PlDataType::UInt8);
        let dt: PlDataType = DataType::U64.into();
        assert_eq!(dt, PlDataType::UInt64);
        let dt: PlDataType = DataType::Usize.into();
        assert_eq!(dt, PlDataType::UInt64);
        let dt: PlDataType = DataType::String.into();
        assert_eq!(dt, PlDataType::String);
        let dt: PlDataType = DataType::Object.into();
        assert!(matches!(dt, PlDataType::Unknown(_)));
    }
    #[test]
    #[cfg(feature = "polars")]
    fn test_from_pldatatype() {
        use tea_deps::polars::prelude::{DataType as PlDataType, UnknownKind};

        assert_eq!(DataType::from(PlDataType::Boolean), DataType::Bool);
        assert_eq!(DataType::from(PlDataType::Float32), DataType::F32);
        assert_eq!(DataType::from(PlDataType::Float64), DataType::F64);
        assert_eq!(DataType::from(PlDataType::Int32), DataType::I32);
        assert_eq!(DataType::from(PlDataType::Int64), DataType::I64);
        assert_eq!(DataType::from(PlDataType::UInt8), DataType::U8);
        assert_eq!(DataType::from(PlDataType::UInt64), DataType::U64);
        assert_eq!(DataType::from(PlDataType::String), DataType::String);

        // For Unknown type, we expect it to be mapped to Unknown
        if let DataType::Unknown = DataType::from(PlDataType::Unknown(UnknownKind::Any)) {
            // Test passes
        } else {
            panic!("Expected Unknown DataType for PlDataType::Object");
        }

        // Test with reference
        assert_eq!(DataType::from(&PlDataType::Boolean), DataType::Bool);
    }
}
