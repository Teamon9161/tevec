use std::borrow::Cow;

use crate::prelude::*;

/// A trait indicating that a type can be referenced to a Trusted and DoubleEnded iterator.
pub trait TIter: GetLen {
    type Item;

    fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = Self::Item>>
    where
        Self: 'a,
        Self::Item: 'a;

    #[inline]
    fn map<U, F>(&self, f: F) -> TrustIter<impl TIterator<Item = U>>
    where
        F: FnMut(Self::Item) -> U,
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

pub struct OptIter<'a, V: Vec1View> {
    pub view: &'a V,
}

impl<V: Vec1View> GetLen for OptIter<'_, V> {
    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }
}

impl<V: Vec1View> TIter for OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = Option<<V::Item as IsNone>::Inner>;

    #[inline]
    fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = Self::Item>>
    where
        Self: 'a,
    {
        TrustIter::new(self.view.titer().map(|v| v.to_opt()), self.len())
    }
}

impl<'a, V: Vec1View> Slice for OptIter<'a, V>
where
    V::Item: IsNone,
{
    type Element = Option<<V::Item as IsNone>::Inner>;
    type Output<'b> = Vec<Option<<V::Item as IsNone>::Inner>> where Self: 'b, Self::Element: 'b;
    #[inline]
    fn slice<'b>(&'b self, start: usize, end: usize) -> TResult<Cow<'a, Self::Output<'b>>>
    where
        Self::Element: 'b,
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

impl<'a, T: IsNone, V: Vec1View<Item = T>> Vec1View for OptIter<'a, V> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<T::Inner> {
        self.view.uget(index).to_opt()
    }
}

// impl<'a, T: IsNone, V: Vec1View<Item = T>> Vec1View for &OptIter<'a, V> {
//     #[inline]
//     unsafe fn uget(&self, index: usize) -> Option<T::Inner> {
//         self.view.uget(index).to_opt()
//     }
// }

impl<'a, V: Vec1View> IntoIterator for &OptIter<'a, V>
where
    V::Item: IsNone,
{
    type Item = Option<<V::Item as IsNone>::Inner>;
    type IntoIter = TrustIter<impl Iterator<Item = Self::Item>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.titer()
    }
}
