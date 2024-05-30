use super::cast::Cast;
use std::cmp::Ordering;
#[cfg(feature = "time")]
use tea_time::{DateTime, TimeDelta};

pub trait IsNone
where
    Self: Sized,
{
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
    fn from_opt(opt: Option<Self::Inner>) -> Self {
        opt.map_or_else(Self::none, Self::from_inner)
    }

    #[inline]
    fn unwrap(self) -> Self::Inner {
        self.to_opt().unwrap()
    }

    #[inline]
    fn not_none(&self) -> bool {
        !self.is_none()
    }

    #[inline]
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        self.to_opt()
            .map(|v| U::from_inner(f(v)))
            .unwrap_or_else(|| U::none())
    }

    #[inline]
    /// let None value be largest, only for sorting(from smallest to largest)
    fn sort_cmp(&self, other: &Self) -> Ordering
    where
        Self: PartialOrd,
    {
        if other.is_none() || (self < other) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    #[inline]
    /// let None value be largest, only for sorting(from largest to smallest)
    fn sort_cmp_rev(&self, other: &Self) -> Ordering
    where
        Self: PartialOrd,
    {
        if other.is_none() || (self > other) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    /// let None value be largest, only for sorting(from smallest to largest)
    #[inline]
    fn sort_cmp_stable(&self, other: &Self) -> Ordering
    where
        Self: PartialOrd,
    {
        if other.is_none() {
            if self.is_none() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if self.is_none() {
            Ordering::Greater
        } else {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
            // let (va, vb) = (self.unwrap(), other.unwrap());
            // if va > vb {
            //     Ordering::Greater
            // } else if va == vb {
            //     Ordering::Equal
            // } else {
            //     Ordering::Less
            // }
        }
    }

    #[inline]
    fn sort_cmp_rev_stable(&self, other: &Self) -> Ordering
    where
        Self: PartialOrd,
    {
        if other.is_none() {
            if self.is_none() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if self.is_none() {
            Ordering::Greater
        } else {
            self.partial_cmp(other).unwrap_or(Ordering::Equal).reverse()
            // let (va, vb) = (self.unwrap(), other.unwrap());
            // if va < vb {
            //     Ordering::Greater
            // } else if va == vb {
            //     Ordering::Equal
            // } else {
            //     Ordering::Less
            // }
        }
    }
}

pub trait IntoCast: IsNone<Inner = Self> + Clone + Sized {
    #[inline]
    fn into_cast<T: IsNone>(self) -> T::Cast<Self>
    where
        T::Inner: Cast<Self::Inner>,
    {
        T::inner_cast(self)
    }
}

impl<U: IsNone<Inner = U> + Clone> IntoCast for U {}

impl IsNone for f32 {
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

    #[inline]
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

impl IsNone for f64 {
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

    #[inline]
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

impl<T: IsNone<Inner = T>> IsNone for Option<T> {
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

    #[inline]
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        self.map(|v| U::from_inner(f(v)))
            .unwrap_or_else(|| U::none())
    }
}

macro_rules! impl_not_none {
    ($($type: ty),*) => {
        $(
            impl IsNone for $type {
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


                #[inline]
                fn map<F, U: IsNone>(self, f: F) -> U
                where
                    F: Fn(Self::Inner) -> U::Inner
                {
                    U::from_inner(f(self))
                }

                #[inline]
                /// only for sorting(from smallest to largest)
                fn sort_cmp(&self, other: &Self) -> Ordering
                where
                    Self: PartialOrd,
                {
                    self.partial_cmp(&other).unwrap()
                }
            }
        )*
    };
}

impl_not_none!(bool, u8, i32, i64, isize, u64, usize);

impl IsNone for String {
    type Inner = String;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
    #[inline]
    fn is_none(&self) -> bool {
        self == "None"
    }

    #[inline]
    fn none() -> Self {
        "None".to_string()
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
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

impl<'a> IsNone for &'a str {
    type Inner = &'a str;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
    #[inline]
    fn is_none(&self) -> bool {
        *self == "None"
    }

    #[inline]
    fn none() -> Self {
        "None"
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
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

#[cfg(feature = "time")]
impl IsNone for DateTime {
    type Inner = DateTime;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nat()
    }

    #[inline]
    fn none() -> Self {
        Self(None)
    }

    #[inline]
    fn to_opt(self) -> Option<Self::Inner> {
        if self.is_nat() {
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
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

#[cfg(feature = "time")]
impl IsNone for TimeDelta {
    type Inner = TimeDelta;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nat()
    }

    #[inline]
    fn none() -> Self {
        Self::nat()
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
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

impl<T> IsNone for Vec<T> {
    type Inner = Vec<T>;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
    #[inline]
    fn is_none(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn none() -> Self {
        Vec::new()
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
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        U::from_inner(f(self))
    }
}

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
