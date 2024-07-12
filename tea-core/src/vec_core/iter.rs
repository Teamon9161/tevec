use std::borrow::Cow;
use std::marker::PhantomData;

use crate::prelude::*;

/// A trait indicating that a type can be referenced to a Trusted and DoubleEnded iterator.
pub trait TIter<T>: GetLen {
    // type Item;

    fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = T>>
    where
        Self: 'a,
        T: 'a;

    #[inline]
    fn map<'a, U, F>(&'a self, f: F) -> TrustIter<impl TIterator<Item = U>>
    where
        F: FnMut(T) -> U,
        T: 'a,
    {
        TrustIter::new(self.titer().map(f), self.len())
    }
}

/// A trait indicating that a type can be converted into a Trusted and DoubleEnded iterator.
pub trait IntoTIter: IntoIterator {
    fn into_titer(self) -> TrustIter<Self::IntoIter>
    where
        Self: Sized;
}

impl<I: IntoIterator + GetLen> IntoTIter for I {
    #[inline]
    fn into_titer(self) -> TrustIter<Self::IntoIter> {
        let len = self.len();
        TrustIter::new(self.into_iter(), len)
    }
}

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

impl<V: Vec1View<T>, T: IsNone> TIter<Option<<T as IsNone>::Inner>> for OptIter<'_, V, T>
// where
//     V::Item: IsNone,
{
    // type Item = Option<<T as IsNone>::Inner>;

    #[inline]
    fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = Option<<T as IsNone>::Inner>>>
    where
        Self: 'a,
    {
        TrustIter::new(self.view.titer().map(|v| v.to_opt()), self.len())
    }
}

impl<'a, V: Vec1View<T>, T: IsNone + 'a> Slice<Option<T::Inner>> for OptIter<'a, V, T>
where
    V::Output<'a>: TIter<T>,
{
    // type Element = Option<T::Inner>;
    type Output<'b> = Vec<Option<T::Inner>> where Self: 'b, Option<T::Inner>: 'b;
    #[inline]
    fn slice<'b>(&'b self, start: usize, end: usize) -> TResult<Cow<'a, Self::Output<'b>>>
    where
        Option<T::Inner>: 'b,
    {
        Ok(Cow::Owned(
            self.view
                .slice(start, end)?
                .titer()
                .map(|v| v.to_opt())
                .collect_trusted_to_vec(),
        ))
    }
}

impl<'a, T: IsNone + 'a, V: Vec1View<T>> Vec1View<Option<T::Inner>> for OptIter<'a, V, T>
where
    for<'b> V::Output<'b>: TIter<T>,
{
    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<T::Inner> {
        self.view.uget(index).to_opt()
    }
}

impl<'a, V: Vec1View<T>, T: IsNone> IntoIterator for &OptIter<'a, V, T>
// where
//     V::Item: IsNone,
{
    type Item = Option<T::Inner>;
    type IntoIter = TrustIter<impl Iterator<Item = Option<T::Inner>>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.titer()
    }
}
