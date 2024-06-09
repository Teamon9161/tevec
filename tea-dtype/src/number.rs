use super::cast::Cast;
use super::isnone::IsNone;
use num_traits::{MulAdd, Num};
use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub, SubAssign};

/// Kahan summation, see https://en.wikipedia.org/wiki/Kahan_summation_algorithm
#[inline]
fn kh_sum<T>(sum: T, v: T, c: &mut T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    let y = v - *c;
    let t = sum + y;
    *c = (t - sum) - y;
    t
}

pub trait Number:
    Copy
    // + Clone
    + Send
    + Sync
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

    fn abs(self) -> Self;

    #[inline(always)]
    fn ceil(self) -> Self {
        self
    }

    #[inline(always)]
    fn floor(self) -> Self {
        self
    }

    #[inline]
    fn min_with(self, other: Self) -> Self {
        if other < self {
            other
        } else {
            self
        }
    }

    #[inline]
    fn max_with(self, other: Self) -> Self {
        if other > self {
            other
        } else {
            self
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

    #[inline(always)]
    #[must_use]
    fn kh_sum(self, v: Self, c: &mut Self) -> Self {
        kh_sum(self, v, c)
    }

    /// if other is nan, then add other to self and n += 1
    /// else just return self
    #[inline]
    fn n_add(self, other: Self, n: &mut usize) -> Self {
        // note: only check if other is NaN
        // assume that self is not NaN
        if other.not_none() {
            *n += 1;
            self + other
        } else {
            self
        }
    }

    /// if other is nan, then product other to self and n += 1
    /// else just return self
    #[inline]
    fn n_prod(self, other: Self, n: &mut usize) -> Self {
        // note: only check if other is NaN
        // assume that self is not NaN
        if other.not_none() {
            *n += 1;
            self * other
        } else {
            self
        }
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

            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }

            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }

        })*
    };
    // special impl for other type
    (signed $($dtype:ty, $datatype:ident); *) => {
        $(impl Number for $dtype {
            impl_number!(@ base_impl $dtype, $datatype);
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
        })*
    };
    // special impl for other type
    (unsigned $($dtype:ty, $datatype:ident); *) => {
        $(impl Number for $dtype {
            impl_number!(@ base_impl $dtype, $datatype);
            #[inline]
            fn abs(self) -> Self {
                self
            }
        })*
    };
}

impl_number!(
    float
    f32, F32;
    f64, F64
);

impl_number!(
    signed
    i32, I32;
    i64, I64
);

impl_number!(
    unsigned
    u64, U64;
    usize, Usize
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ceil() {
        fn _ceil<T: Number>(v: T) -> T {
            v.ceil()
        }
        assert_eq!(_ceil(1.23_f64), 2.);
        assert_eq!(_ceil(-1.23_f32), -1.);
        assert_eq!(_ceil(0_usize), 0);
        assert_eq!(_ceil(-3i32), -3);
    }

    #[test]
    fn test_floor() {
        fn _floor<T: Number>(v: T) -> T {
            v.floor()
        }
        assert_eq!(_floor(1.23_f64), 1.);
        assert_eq!(_floor(-1.23_f32), -2.);
        assert_eq!(_floor(0_usize), 0);
        assert_eq!(_floor(-3i32), -3);
    }
}
