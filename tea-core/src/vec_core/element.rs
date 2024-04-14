#[cfg(feature = "pl")]
use crate::backends_impl::PlTypeMap;

#[cfg(feature = "time")]
use tea_dtype::{DateTime, TimeDelta};

#[cfg(feature = "pl")]
pub trait Element: PlTypeMap + Clone {}

#[cfg(not(feature = "pl"))]
pub trait Element: Clone {}

macro_rules! impl_element {
    ($($type: ty),*) => {
        $(
            impl Element for $type {}
            impl Element for Option<$type> {}
        )*
    };
}

impl_element!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, char);

#[cfg(feature = "time")]
impl_element!(DateTime, TimeDelta);
