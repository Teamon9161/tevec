use super::{TrustIter, Vec1View};
use tea_dtype::{Cast, IsNone};

pub trait Length {
    fn len(&self) -> usize;
}

pub trait ToIter {
    type Item;

    fn len(&self) -> usize;

    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self::Item: 'a;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn map<U, F>(&self, f: F) -> std::iter::Map<TrustIter<impl Iterator<Item = Self::Item>>, F>
    where
        F: FnMut(Self::Item) -> U,
    {
        self.to_iterator().map(f)
    }
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

    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self::Item: 'a,
    {
        TrustIter::new(self.view.to_iterator().map(|v| v.cast()), self.len())
    }
}

impl<V: Vec1View> ToIter for &OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = <V::Item as IsNone>::Opt;

    #[inline]
    fn len(&self) -> usize {
        self.view.len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self::Item: 'a,
    {
        TrustIter::new(self.view.to_iter().map(|v| v.cast()), self.view.len())
    }
}

impl<'a, T: IsNone + Clone, V: Vec1View<Item = T>> Vec1View for OptIter<'a, V>
where
    T::Opt: Clone,
{
    // #[inline]
    // fn len(&self) -> usize {
    //     self.len()
    // }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T::Opt {
        self.view.uget(index).cast()
    }
}

impl<'a, T: IsNone + Clone, V: Vec1View<Item = T>> Vec1View for &OptIter<'a, V>
where
    T::Opt: Clone,
{
    // #[inline]
    // fn len(&self) -> usize {
    //     self.view.len()
    // }

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
    type IntoIter = TrustIter<impl Iterator<Item = Self::Item>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.to_iter()
        // Vec1View::to_iter(self)
    }
}
