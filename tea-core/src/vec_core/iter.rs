use super::{TrustIter, TrustedLen, Vec1View};
use tea_dtype::{Cast, IsNone};

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
    fn map<U, F>(&self, f: F) -> TrustIter<impl Iterator<Item = U>>
    where
        F: FnMut(Self::Item) -> U,
    {
        TrustIter::new(self.to_iterator().map(f), self.len())
    }
}

pub trait IntoIter: Iterator + TrustedLen
where
    Self: Sized,
{
    fn len(&self) -> usize;

    fn into_iterator(self) -> TrustIter<Self>;

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

// impl<V: Vec1View> IntoIter for &V {
//     type Item = V::Item;
//     type IntoIter<'a> = V::IntoIter<'a> where Self: 'a;

//     #[inline]
//     fn len(&self) -> usize {
//         (*self).len()
//     }

//     #[inline]
//     fn into_iterator<'a>(self) -> TrustIter<Self::IntoIter<'a>> where Self: 'a {
//         self.to_iter()
//     }
// }

pub struct OptIter<'a, V: Vec1View> {
    pub view: &'a V,
}

impl<V: Vec1View> ToIter for OptIter<'_, V>
where
    V::Item: IsNone,
{
    type Item = <V::Item as IsNone>::Opt;
    // type IntoIter<'a> = impl Iterator<Item = Self::Item> where Self: 'a;

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
    }
}
