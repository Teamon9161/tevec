use polars::prelude::*;
#[cfg(feature = "time")]
use tea_dtype::{DateTime, TimeDelta};

pub trait PlTypeMap
where
    Self: Clone + Sized,
{
    type Map: PolarsDataType;
}

macro_rules! impl_pl_type_map {
    ($($type: ty: $pl_type: ty),*) => {
        $(
            impl PlTypeMap for $type {
                type Map = $pl_type;
            }

            impl PlTypeMap for Option<$type> {
                type Map = $pl_type;
            }
        )*
    };

    (default $($type: ty),*) => {
        $(
            impl PlTypeMap for $type {
                type Map = BooleanType;
            }

            impl PlTypeMap for Option<$type> {
                type Map = BooleanType;
            }
        )*
    };
}

impl_pl_type_map!(
    i8: Int8Type,
    i16: Int16Type,
    i32: Int32Type,
    i64: Int64Type,
    u8: UInt8Type,
    u16: UInt16Type,
    u32: UInt32Type,
    u64: UInt64Type,
    f32: Float32Type,
    f64: Float64Type,
    bool: BooleanType,
    String: StringType
);

impl_pl_type_map!(default char, isize, usize);

#[cfg(feature = "time")]
impl_pl_type_map!(
    DateTime: DatetimeType,
    TimeDelta: DurationType
);
