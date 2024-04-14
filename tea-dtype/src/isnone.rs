use super::cast::Cast;

pub trait Opt {
    fn v(self) -> Self;
}

impl<T> Opt for Option<T> {
    #[inline(always)]
    fn v(self) -> Self {
        self
    }
}

pub trait IsNone
where
    Self: Sized,
    Self: Cast<Self::Opt>,
    Self::Opt: Cast<Self> + Opt,
{
    type Opt;

    fn is_none(&self) -> bool;

    fn none() -> Self;

    #[inline]
    fn not_none(&self) -> bool {
        !self.is_none()
    }
}

impl IsNone for f32 {
    type Opt = Option<f32>;
    #[inline]
    #[allow(clippy::eq_op)]
    fn is_none(&self) -> bool {
        self != self
    }

    #[inline]
    fn none() -> Self {
        f32::NAN
    }

    #[inline]
    #[allow(clippy::eq_op)]
    fn not_none(&self) -> bool {
        self == self
    }
}

impl IsNone for f64 {
    type Opt = Option<f64>;
    #[inline]
    #[allow(clippy::eq_op)]
    fn is_none(&self) -> bool {
        self != self
    }

    #[inline]
    fn none() -> Self {
        f64::NAN
    }

    #[inline]
    #[allow(clippy::eq_op)]
    fn not_none(&self) -> bool {
        self == self
    }
}

impl<T> IsNone for Option<T> {
    type Opt = Option<T>; // Option<Option<T>> is not needed
    #[inline]
    fn is_none(&self) -> bool {
        self.is_none()
    }

    #[inline]
    fn none() -> Self {
        None
    }

    #[inline]
    fn not_none(&self) -> bool {
        self.is_some()
    }
}

macro_rules! impl_not_none {
    ($($type: ty),*) => {
        $(
            impl IsNone for $type {
                type Opt = Option<$type>;
                #[inline]
                #[allow(clippy::eq_op)]
                fn is_none(&self) -> bool {
                    false
                }

                fn none() -> Self {
                    panic!("Cannot call none() on a non-float type");
                }

                #[inline]
                #[allow(clippy::eq_op)]
                fn not_none(&self) -> bool {
                    true
                }
            }
        )*
    };
}

impl_not_none!(bool, i32, i64, u64, usize);
