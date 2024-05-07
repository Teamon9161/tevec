use super::{TrustIter, TrustedLen, Vec1View};
use tea_dtype::IsNone;

pub trait ToIter {
    type Item;

    fn len(&self) -> usize;

    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self: 'a,
        Self::Item: 'a;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn map<U, F>(&self, f: F) -> TrustIter<impl Iterator<Item = U>>
    where
        F: FnMut(Self::Item) -> U,
    {
        TrustIter::new(self.to_iterator().map(f), self.len())
    }
}

pub trait IntoIter: TrustedLen // where
//     Self: Sized,
{
    fn len(&self) -> usize;

    fn into_iterator(self) -> TrustIter<Self>
    where
        Self: Sized;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<I: Iterator + TrustedLen> IntoIter for I {
    #[inline]
    fn len(&self) -> usize {
        self.size_hint().1.unwrap()
    }

    #[inline]
    fn into_iterator(self) -> TrustIter<I> {
        let len = self.len();
        TrustIter::new(self.into_iter(), len)
    }
}

pub struct OptIter<'a, V: Vec1View> {
    pub view: &'a V,
}

impl<V: Vec1View> ToIter for OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = Option<<V::Item as IsNone>::Inner>;

    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self: 'a,
    {
        TrustIter::new(self.view.to_iterator().map(|v| v.to_opt()), self.len())
    }
}

impl<V: Vec1View> ToIter for &OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = Option<<V::Item as IsNone>::Inner>;

    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self: 'a,
    {
        TrustIter::new(self.view.to_iterator().map(|v| v.to_opt()), self.len())
    }
}

impl<'a, T: IsNone, V: Vec1View<Item = T>> Vec1View for OptIter<'a, V> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<T::Inner> {
        self.view.uget(index).to_opt()
    }
}

impl<'a, T: IsNone, V: Vec1View<Item = T>> Vec1View for &OptIter<'a, V> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<T::Inner> {
        self.view.uget(index).to_opt()
    }
}

impl<'a, V: Vec1View> IntoIterator for &OptIter<'a, V>
where
    V::Item: IsNone,
{
    type Item = Option<<V::Item as IsNone>::Inner>;
    type IntoIter = TrustIter<impl Iterator<Item = Self::Item>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.to_iter()
    }
}
