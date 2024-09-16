use std::marker::PhantomData;

use crate::prelude::*;

/// A trait indicating that a type can be referenced to a Trusted and DoubleEnded iterator.
/// A trait for types that can be iterated over with a trusted iterator.
///
/// This trait extends the `GetLen` trait and provides methods to create
/// trusted iterators over the implementing type.
///
/// # Type Parameters
///
/// * `T`: The type of items yielded by the iterator.
pub trait TIter<T>: GetLen {
    /// Creates a trusted iterator over the items of this collection.
    ///
    /// # Returns
    ///
    /// An iterator that implements the `TIterator` trait, yielding items of type `T`.
    fn titer(&self) -> impl TIterator<Item = T> + '_;

    /// Maps each item in the collection using the provided function.
    ///
    /// This method creates a new iterator that applies the given function to
    /// each item yielded by the original iterator.
    ///
    /// # Arguments
    ///
    /// * `f`: A function that takes an item of type `T` and returns an item of type `U`.
    ///
    /// # Returns
    ///
    /// An iterator that yields the mapped items.
    ///
    /// # Type Parameters
    ///
    /// * `'a`: The lifetime of the reference to `self`.
    /// * `U`: The type of items yielded by the new iterator.
    /// * `F`: The type of the mapping function.
    #[inline]
    fn map<'a, U, F>(&'a self, f: F) -> impl TIterator<Item = U>
    where
        F: FnMut(T) -> U,
        T: 'a,
    {
        self.titer().map(f)
    }
}

/// A trait indicating that a type can be converted into a Trusted and DoubleEnded iterator.
pub trait IntoTIter: IntoIterator
where
    Self::IntoIter: TIterator,
{
    fn into_titer(self) -> Self::IntoIter
    where
        Self: Sized;
}

impl<I: IntoIterator + GetLen> IntoTIter for I
where
    Self::IntoIter: TIterator,
{
    #[inline]
    fn into_titer(self) -> Self::IntoIter {
        self.into_iter()
    }
}
/// An iterator adapter that wraps a `Vec1View` and converts its items to `Option` types.
///
/// This struct provides a way to iterate over a `Vec1View` while converting each item
/// to an `Option` type. It's particularly useful when working with types that implement
/// the `IsNone` trait, allowing for a uniform representation of potentially absent values.
///
/// # Type Parameters
///
/// * `'a`: The lifetime of the reference to the underlying `Vec1View`.
/// * `V`: The type of the underlying `Vec1View`.
/// * `T`: The item type of the `Vec1View`.
///
/// # Fields
///
/// * `view`: A reference to the underlying `Vec1View`.
/// * `item`: A `PhantomData` to carry the item type `T`.
pub struct OptIter<'a, V: Vec1View<T>, T> {
    pub view: &'a V,
    pub item: PhantomData<T>,
}

impl<V: Vec1View<T>, T> GetLen for OptIter<'_, V, T> {
    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }
}

impl<V: Vec1View<T>, T: IsNone> TIter<Option<<T as IsNone>::Inner>> for OptIter<'_, V, T> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = Option<<T as IsNone>::Inner>> {
        self.view.titer().map(|v| v.to_opt())
    }
}

impl<'a, T: IsNone + 'a, V: Vec1View<T>> Vec1View<Option<T::Inner>> for OptIter<'a, V, T>
where
    for<'b> V::SliceOutput<'b>: TIter<T>,
{
    type SliceOutput<'b> = Vec<Option<T::Inner>> where Self: 'b;

    #[inline]
    fn slice<'b>(&'b self, start: usize, end: usize) -> TResult<Self::SliceOutput<'b>>
    where
        T: 'b,
    {
        Ok(self
            .view
            .slice(start, end)?
            .titer()
            .map(|v| v.to_opt())
            .collect_trusted_to_vec())
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        self.view.get_backend_name()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<T::Inner> {
        self.view.uget(index).to_opt()
    }
}

impl<'a, 'b, V: Vec1View<T>, T: IsNone> IntoIterator for &'b OptIter<'a, V, T> {
    type Item = Option<T::Inner>;
    type IntoIter = Box<dyn TrustedLen<Item = Option<T::Inner>> + 'b>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.titer())
    }
}
