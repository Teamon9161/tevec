use num_traits::{Num, MulAdd};
use std::cmp::PartialOrd;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use super::isnone::IsNone;
use super::cast::Cast;

#[cfg(feature = "time")]
pub use tea_time::{DateTime, TimeDelta, TimeUnit};

pub trait Number:
    Copy
    + Clone
    + IsNone
    + Sized
    + Default
    + Num
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + PartialOrd
    + MulAdd
    + Cast<f64>
    + Cast<f32>
    + Cast<usize>
    + Cast<i32>
    + Cast<i64>
    + 'static
{
    // type Dtype;
    /// return the min value of the data type
    fn min_() -> Self;

    /// return the max value of the data type
    fn max_() -> Self;

    fn min_with(&self, other: &Self) -> Self {
        if other < self {
            *other
        } else {
            *self
        }
    }

    fn max_with(&self, other: &Self) -> Self {
        if other > self {
            *other
        } else {
            *self
        }
    }

    #[inline(always)]
    fn f32(self) -> f32 {
        Cast::<f32>::cast(self)
    }

    #[inline(always)]
    fn f64(self) -> f64 {
        Cast::<f64>::cast(self)
    }

    #[inline(always)]
    fn i32(self) -> i32 {
        Cast::<i32>::cast(self)
    }

    #[inline(always)]
    fn i64(self) -> i64 {
        Cast::<i64>::cast(self)
    }

    #[inline(always)]
    fn usize(self) -> usize {
        Cast::<usize>::cast(self)
    }

    /// create a value of type T using a value of type U using `Cast`
    #[inline(always)]
    fn fromas<U>(v: U) -> Self
    where
        U: Number + Cast<Self>,
        Self: 'static,
    {
        v.to::<Self>()
    }

    /// cast self to another dtype using `Cast`
    #[inline(always)]
    fn to<T: Number>(self) -> T
    where
        Self: Cast<T>,
    {
        Cast::<T>::cast(self)
    }
}

macro_rules! impl_number {
    (@ base_impl $dtype:ty, $datatype:ident) => {

        #[inline(always)]
        fn min_() -> $dtype {
            <$dtype>::MIN
        }

        #[inline(always)]
        fn max_() -> $dtype {
            <$dtype>::MAX
        }

    };
    // special impl for float
    (float $($dtype:ty, $datatype:ident); *) => {
        $(impl Number for $dtype {
            impl_number!(@ base_impl $dtype, $datatype);
        })*
    };
    // special impl for other type
    (other $($dtype:ty, $datatype:ident); *) => {
        $(impl Number for $dtype {
            impl_number!(@ base_impl $dtype, $datatype);
        })*
    };
}

impl_number!(
    float
    f32, F32;
    f64, F64
);

impl_number!(
    other
    i32, I32;
    i64, I64;
    u64, U64;
    usize, Usize
);

pub trait BoolType {
    fn bool_(self) -> bool;
}

impl BoolType for bool {
    fn bool_(self) -> bool {
        self
    }
}

