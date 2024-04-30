use super::cast::Cast;

pub trait Opt: IsNone {
    type Value;
    fn v(self) -> Self::Value;

    fn map_to<U, F>(self, f: F) -> Option<U>
    where
        F: FnMut(Self::Value) -> U;
}

impl<T: IsNone> Opt for Option<T> {
    type Value = T;

    #[inline(always)]
    fn v(self) -> T {
        self.unwrap()
    }

    #[inline]
    fn map_to<U, F>(self, f: F) -> Option<U>
    where
        F: FnMut(Self::Value) -> U,
    {
        self.map(f)
    }
}

pub trait IsNone
where
    Self: Sized,
    Self: Cast<Self::Opt>,
{
    type Opt;
    type Inner: IsNone;
    type Cast<U: IsNone>: IsNone;

    fn is_none(&self) -> bool;

    fn none() -> Self;

    fn to_opt(self) -> Option<Self::Inner>;

    fn from_inner(inner: Self::Inner) -> Self;

    fn inner_cast<U: IsNone>(inner: U) -> Self::Cast<U>
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
    type Cast<U: IsNone> = U;

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
    fn inner_cast<U: IsNone>(inner: U) -> Self::Cast<U>
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
    type Cast<U: IsNone> = U;
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
    fn inner_cast<U: IsNone>(inner: U) -> Self::Cast<U>
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

impl<T: IsNone> IsNone for Option<T> {
    type Opt = Option<T>; // Option<Option<T>> is not needed
    type Inner = T;
    type Cast<U: IsNone> = Option<U>;
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
    fn inner_cast<U: IsNone>(inner: U) -> Self::Cast<U>
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
                type Cast<U: IsNone> = U;
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
                fn inner_cast<U: IsNone>(inner: U) -> Self::Cast<U>
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
