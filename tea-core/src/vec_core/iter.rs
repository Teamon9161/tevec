use super::{TrustIter, Vec1View};
use tea_dtype::{Cast, IsNone};

pub trait ToIter {
    type Item;
    fn to_iterator<'a>(&'a self) -> impl Iterator<Item = Self::Item>
    where
        Self::Item: 'a;
}

pub trait IntoIter<T> {
    fn into_iterator(self) -> impl Iterator<Item = T>;
}

pub struct OptIter<'a, V: Vec1View> {
    pub view: &'a V,
}

impl<V: Vec1View> ToIter for OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = <V::Item as IsNone>::Opt;
    fn to_iterator<'a>(&'a self) -> impl Iterator<Item = Self::Item>
    where
        Self::Item: 'a,
    {
        self.view.to_iterator().map(|v| v.cast())
    }
}

impl<V: Vec1View> ToIter for &OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = <V::Item as IsNone>::Opt;
    fn to_iterator<'a>(&'a self) -> impl Iterator<Item = Self::Item>
    where
        Self::Item: 'a,
    {
        self.view.to_iter().map(|v| v.cast())
    }
}

impl<'a, T: IsNone + Clone, V: Vec1View<Item = T>> Vec1View for OptIter<'a, V>
where
    T::Opt: Clone,
{
    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T::Opt {
        self.view.uget(index).cast()
    }
}

impl<'a, T: IsNone + Clone, V: Vec1View<Item = T>> Vec1View for &OptIter<'a, V>
where
    T::Opt: Clone,
{
    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T::Opt {
        self.view.uget(index).cast()
    }
}

impl<'a, V: Vec1View> IntoIterator for &OptIter<'a, V>
where
    V::Item: IsNone + Clone,
    <V::Item as IsNone>::Opt: Clone,
{
    type Item = <V::Item as IsNone>::Opt;
    type IntoIter = TrustIter<impl Iterator<Item = Self::Item>, Self::Item>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.to_iter()
        // Vec1View::to_iter(self)
    }
}
