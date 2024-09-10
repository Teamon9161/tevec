use std::cmp::Ordering;

#[cfg(feature = "time")]
use tea_time::{DateTime, Time, TimeDelta, TimeUnitTrait};

use super::cast::Cast;
use super::number::Number;

/// A trait for types that can represent a "none" or null-like state.
///
/// This trait is implemented by types that can be in a "none" state,
/// similar to Rust's `Option` type. It provides methods for working
/// with such types in a generic way.
///
/// # Type Parameters
///
/// * `Inner`: The inner type. For regular types `T`, `Inner` is `T` itself.
///   For `Option<T>`, `Inner` is `T`. This allows `Option<T::Inner>` to be
///   the same type for both `T` and `Option<T>`.
///
/// * `Cast<U>`: A type that represents the result of casting to another `IsNone` type.
///   For regular types, `Cast<U>` is typically `U`. For `Option<T>`, `Cast<U>`
///   is `Option<U>`. This allows operations like `Option<T>::cast<f64>()` to result
///   in `Option<f64>`, while `T::cast<f64>()` results in `f64`.
///
/// # Required Methods
///
/// Implementors must define:
/// - `is_none`: Check if the value is in the "none" state.
/// - `none`: Create a new instance in the "none" state.
/// - `to_opt`: Convert to an `Option<Inner>`.
/// - `as_opt`: Get a reference as an `Option<&Inner>`.
/// - `from_inner`: Create an instance from an `Inner` value.
/// - `inner_cast`: Cast between different `IsNone` types.
///
/// # Provided Methods
///
/// The trait also provides default implementations for:
/// - `from_opt`: Create an instance from an `Option<Inner>`.
/// - `unwrap`: Get the inner value, panicking if none.
/// - `not_none`: Check if the value is not in the "none" state.
/// - `map`: Apply a function to the inner value if not none.
pub trait IsNone: Clone
where
    Self: Sized,
{
    type Inner: IsNone<Inner = Self::Inner>;
    type Cast<U: IsNone<Inner = U>>: IsNone<Inner = U>;

    /// Checks if the value is in the "none" state.
    ///
    /// # Returns
    ///
    /// `true` if the value is in the "none" state, `false` otherwise.
    fn is_none(&self) -> bool;

    /// Creates a new instance of `Self` in the "none" state.
    ///
    /// This method is used to generate a value that represents the absence of a valid value,
    /// similar to `None` in Rust's `Option` type.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Self` that is considered to be in the "none" state.
    // TODO: some type doesn't have none value, so it should return a `TResult<Self>`
    fn none() -> Self;

    /// Converts the value to an `Option<Self::Inner>`.
    ///
    /// This method transforms the current value into an `Option` type, where:
    /// - If the current value is in the "none" state, it returns `None`.
    /// - Otherwise, it returns `Some(inner)`, where `inner` is the wrapped value.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Inner>` representing the current value.
    fn to_opt(self) -> Option<Self::Inner>;

    /// Converts the value to an `Option<&Self::Inner>`.
    ///
    /// This method returns a reference to the inner value as an `Option` type, where:
    /// - If the current value is in the "none" state, it returns `None`.
    /// - Otherwise, it returns `Some(&inner)`, where `inner` is a reference to the wrapped value.
    ///
    /// # Returns
    ///
    /// An `Option<&Self::Inner>` representing a reference to the current value.
    fn as_opt(&self) -> Option<&Self::Inner>;

    /// Creates a new instance of `Self` from the given inner value.
    ///
    /// This method converts a value of type `Self::Inner` into `Self`.
    /// For example, if `Self` is `f64`, `Self::Inner` is also `f64`, so this method
    /// would essentially be an identity function. However, if `Self` is `Option<f64>`,
    /// `Self::Inner` would be `f64`, so this method would wrap the `f64` value in `Some`.
    ///
    /// # Arguments
    ///
    /// * `inner` - The inner value to be converted.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Self` created from the provided inner value.
    fn from_inner(inner: Self::Inner) -> Self;

    /// Casts the inner type of `Self` to a new type `U`.
    ///
    /// This method allows for casting between different `IsNone` types, preserving the "none" state
    /// if applicable. It uses the `Cast` trait to perform the actual type conversion.
    ///
    /// # Type Parameters
    ///
    /// * `U`: The target type, which must implement `IsNone` and have `Inner = U`.
    ///
    /// # Arguments
    ///
    /// * `inner`: The value of type `U` to be cast.
    ///
    /// # Returns
    ///
    /// Returns `Self::Cast<U>`, which is the result of casting `Self` to a type that can hold `U`.
    fn inner_cast<U: IsNone<Inner = U>>(inner: U) -> Self::Cast<U>
    where
        Self::Inner: Cast<U::Inner>;

    #[inline]
    /// Creates a new instance of `Self` from an `Option<Self::Inner>`.
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

    /// Maps a function over the inner value of `Self`, if it exists.
    ///
    /// This method applies a given function to the inner value of `Self` if it's not none,
    /// and wraps the result in a new `IsNone` type `U`. If `Self` is none, it returns
    /// the none value of type `U`.
    ///
    /// # Type Parameters
    ///
    /// * `F`: The type of the mapping function.
    /// * `U`: The target `IsNone` type.
    ///
    /// # Arguments
    ///
    /// * `self`: The `IsNone` value to map over.
    /// * `f`: A function that takes `Self::Inner` and returns `U::Inner`.
    ///
    /// # Returns
    ///
    /// Returns a new `IsNone` value of type `U`, which is either:
    /// - The result of applying `f` to the inner value, wrapped in `U`, if `self` is not none.
    /// - The none value of `U`, if `self` is none.
    #[inline]
    fn map<F, U: IsNone>(self, f: F) -> U
    where
        F: Fn(Self::Inner) -> U::Inner,
    {
        self.to_opt()
            .map(|v| U::from_inner(f(v)))
            .unwrap_or_else(|| U::none())
    }

    /// Computes the absolute value of the inner value, if it exists.
    ///
    /// This method applies the absolute value function to the inner value of `Self` if it's not none.
    /// If `Self` is none, it returns the none value.
    ///
    /// # Type Constraints
    ///
    /// * `Self::Inner`: Must implement the `Number` trait, which provides the `abs()` method.
    ///
    /// # Returns
    ///
    /// Returns a new `Self` instance containing:
    /// - The absolute value of the inner value, if `self` is not none.
    /// - The none value of `Self`, if `self` is none.
    #[inline]
    fn vabs(self) -> Self
    where
        Self::Inner: Number,
    {
        self.map(|v| v.abs())
    }

    #[inline]
    /// Compares two values for sorting, treating `None` as the largest value.
    ///
    /// This method is designed for sorting `Some` values from smallest to largest,
    /// with `None` values considered larger than any non-`None` value.
    ///
    /// # Arguments
    ///
    /// * `self` - The first `IsNone` value to compare.
    /// * `other` - The second `IsNone` value to compare.
    ///
    /// # Returns
    ///
    /// Returns an `Ordering` that can be used for sorting:
    /// - `Ordering::Less` if `self` is less than `other`.
    /// - `Ordering::Equal` if `self` is equal to `other`.
    /// - `Ordering::Greater` if `self` is greater than `other` or if `self` is `None`.
    ///
    /// # Type Constraints
    ///
    /// * `Self::Inner`: Must implement the `PartialOrd` trait.
    ///
    /// # Notes
    ///
    /// - If both values are `Some`, their inner values are compared using `partial_cmp`.
    /// - If the inner values can't be compared (e.g., NaN for floats), `None` is considered greater.
    /// - If both values are `None`, they are considered equal.
    /// - A `None` value is always considered greater than any `Some` value.
    fn sort_cmp(&self, other: &Self) -> Ordering
    where
        Self::Inner: PartialOrd,
    {
        match (self.as_opt(), other.as_opt()) {
            (Some(va), Some(vb)) => va.partial_cmp(vb).unwrap_or_else(|| {
                if va.is_none() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }),
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Greater,
            (_, None) => Ordering::Less,
        }
    }

    #[inline]
    /// Compares two values for reverse sorting, treating `None` as the largest value.
    ///
    /// This method is designed for sorting `Some` values from largest to smallest,
    /// with `None` values considered larger than any non-`None` value.
    ///
    /// # Arguments
    ///
    /// * `self` - The first `IsNone` value to compare.
    /// * `other` - The second `IsNone` value to compare.
    ///
    /// # Returns
    ///
    /// Returns an `Ordering` that can be used for reverse sorting:
    /// - `Ordering::Less` if `self` is greater than `other`.
    /// - `Ordering::Equal` if `self` is equal to `other`.
    /// - `Ordering::Greater` if `self` is less than `other` or if `self` is `None`.
    ///
    /// # Type Constraints
    ///
    /// * `Self::Inner`: Must implement the `PartialOrd` trait.
    ///
    /// # Notes
    ///
    /// - If both values are `Some`, their inner values are compared using `partial_cmp` and then reversed.
    /// - If the inner values can't be compared (e.g., NaN for floats), `None` is considered greater.
    /// - If both values are `None`, they are considered equal.
    /// - A `None` value is always considered greater than any `Some` value.
    fn sort_cmp_rev(&self, other: &Self) -> Ordering
    where
        Self::Inner: PartialOrd,
    {
        match (self.as_opt(), other.as_opt()) {
            (Some(va), Some(vb)) => va
                .partial_cmp(vb)
                .unwrap_or_else(|| {
                    if va.is_none() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .reverse(),
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Greater,
            (_, None) => Ordering::Less,
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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
    fn as_opt(&self) -> Option<&Self::Inner> {
        self.as_ref()
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

                #[inline]
                fn as_opt(&self) -> Option<&Self::Inner> {
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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
impl<Unit: TimeUnitTrait> IsNone for DateTime<Unit> {
    type Inner = DateTime<Unit>;
    type Cast<U: IsNone<Inner = U> + Clone> = U;
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nat()
    }

    #[inline]
    fn none() -> Self {
        DateTime::nat()
    }

    #[inline]
    fn to_opt(self) -> Option<Self::Inner> {
        if self.is_nat() {
            None
        } else {
            Some(self)
        }
    }

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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
impl IsNone for Time {
    type Inner = Time;
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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

impl<T: Clone> IsNone for Vec<T> {
    type Inner = Vec<T>;
    type Cast<U: IsNone<Inner = U>> = U;
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

    #[inline]
    fn as_opt(&self) -> Option<&Self::Inner> {
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
    fn test_str() {
        let a = "dsf";
        assert_eq!(a.unwrap(), "dsf");
        assert_eq!(a.to_opt(), Some("dsf"));
        let a = Some("dsf");
        assert_eq!(a.to_opt(), Some("dsf"));
    }

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

    #[test]
    fn test_unwrap() {
        let v = Some(f64::NAN);
        assert!(v.not_none());
        assert!(IsNone::unwrap(v).is_nan());
        let v: Option<i32> = None;
        assert!(!v.not_none());
        let v = f64::NAN;
        assert!(v.is_none());
        let v = 1;
        assert!(!v.is_none());
    }
}
