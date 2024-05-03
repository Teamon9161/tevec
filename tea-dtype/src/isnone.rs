use super::cast::Cast;

pub trait Opt: IsNone {
    fn v(self) -> Self::Inner;

    fn map_to<U, F>(self, f: F) -> Option<U>
    where
        F: FnMut(Self::Inner) -> U;
}

impl<T: IsNone<Inner = T>> Opt for Option<T> {
    #[inline(always)]
    fn v(self) -> T {
        self.unwrap()
    }

    #[inline]
    fn map_to<U, F>(self, f: F) -> Option<U>
    where
        F: FnMut(T) -> U,
    {
        self.map(f)
    }
}

pub trait IsNone
where
    Self: Sized,
    Self: Cast<Self::Opt>,
{
    type Opt: Opt<Inner = Self::Inner>;
    type Inner: IsNone<Inner = Self::Inner>;
    type Cast<U: IsNone<Inner = U> + Clone>: IsNone<Inner = U> + Clone;

    fn is_none(&self) -> bool;

    fn none() -> Self;

    fn to_opt(self) -> Option<Self::Inner>;

    fn from_inner(inner: Self::Inner) -> Self;

    fn inner_cast<U: IsNone<Inner = U> + Clone>(inner: U) -> Self::Cast<U>
    where
        Self::Inner: Cast<U::Inner>;

    #[inline]
    fn unwrap(self) -> Self::Inner {
        self.to_opt().unwrap()
    }

    #[inline]
    fn not_none(&self) -> bool {
        !self.is_none()
    }
}

impl IsNone for f32 {
    type Opt = Option<f32>;
    type Inner = f32;
    type Cast<U: IsNone<Inner = U> + Clone> = U;

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
    fn to_opt(self) -> Option<Self::Inner> {
        if self.is_none() {
            None
        } else {
            Some(self)
        }
    }

    #[inline(always)]
    fn from_inner(inner: Self::Inner) -> Self {
        inner
    }

    #[inline]
    fn inner_cast<U: IsNone<Inner = U> + Clone>(inner: U) -> Self::Cast<U>
    where
        Self::Inner: Cast<U::Inner>,
    {
        Cast::<U>::cast(inner)
    }

    #[inline(always)]
    fn unwrap(self) -> Self::Inner {
        self
    }

    #[inline]
    #[allow(clippy::eq_op)]
    fn not_none(&self) -> bool {
        self == self
    }
}

impl IsNone for f64 {
    type Opt = Option<f64>;
    type Inner = f64;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
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
    fn to_opt(self) -> Option<Self::Inner> {
        if self.is_none() {
            None
        } else {
            Some(self)
        }
    }

    #[inline(always)]
    fn from_inner(inner: Self::Inner) -> Self {
        inner
    }

    #[inline]
    fn inner_cast<U: IsNone<Inner = U> + Clone>(inner: U) -> Self::Cast<U>
    where
        Self::Inner: Cast<U::Inner>,
    {
        Cast::<U>::cast(inner)
    }

    #[inline(always)]
    fn unwrap(self) -> Self::Inner {
        self
    }
    #[inline]
    #[allow(clippy::eq_op)]
    fn not_none(&self) -> bool {
        self == self
    }
}

impl<T: IsNone<Inner = T>> IsNone for Option<T> {
    type Opt = Option<T>; // Option<Option<T>> is not needed
    type Inner = T;
    type Cast<U: IsNone<Inner = U> + Clone> = Option<U>;
    #[inline]
    fn is_none(&self) -> bool {
        self.is_none()
    }

    #[inline]
    fn none() -> Self {
        None
    }

    #[inline(always)]
    fn to_opt(self) -> Option<Self::Inner> {
        self
    }

    #[inline]
    fn from_inner(inner: Self::Inner) -> Self {
        if inner.is_none() {
            None
        } else {
            Some(inner)
        }
    }

    #[inline]
    fn inner_cast<U: IsNone<Inner = U> + Clone>(inner: U) -> Self::Cast<U>
    where
        Self::Inner: Cast<U::Inner>,
    {
        if inner.is_none() {
            None
        } else {
            Some(Cast::<U>::cast(inner))
        }
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
                type Inner = $type;
                type Cast<U: IsNone<Inner=U> + Clone> = U;
                #[inline]
                #[allow(clippy::eq_op)]
                fn is_none(&self) -> bool {
                    false
                }

                fn none() -> Self {
                    panic!("Cannot call none() on a non-float type");
                }

                #[inline(always)]
                fn to_opt(self) -> Option<Self::Inner> {
                    Some(self)
                }

                #[inline(always)]
                fn from_inner(inner: Self::Inner) -> Self {
                    inner
                }


                #[inline]
                fn inner_cast<U: IsNone<Inner=U> + Clone>(inner: U) -> Self::Cast<U>
                where Self::Inner: Cast<U::Inner>
                {
                    Cast::<U>::cast(inner)
                }


                #[inline(always)]
                fn unwrap(self) -> Self::Inner {
                    self
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

#[cfg(test)]
mod tests {
    use crate::{Cast, IsNone};
    #[test]
    fn test_type_cast() {
        fn test1<T: IsNone>(_v: T) -> f64
        where
            T::Inner: Cast<f64>,
        {
            let out = T::inner_cast(0.);
            out.unwrap()
        }
        assert_eq!(0., test1(2_i32));
        assert_eq!(0., test1(Some(3_usize)));

        fn test2<T: IsNone>(_v: T) -> T::Cast<f64>
        where
            T::Inner: Cast<f64>,
        {
            T::inner_cast(0.)
        }
        assert_eq!(0., test2(2_i32));
        assert_eq!(Some(0.), test2(Some(3_usize)));

        fn test3<T: IsNone>(_v: T) -> f64
        where
            T::Inner: Cast<f64>,
        {
            let out = T::inner_cast(0.);
            let v = out.unwrap();
            if v > 1. {
                v + 1.
            } else {
                v + 2.
            }
        }
        assert_eq!(2., test3(1_i32));
    }
}
