use std::mem::MaybeUninit;

use crate::prelude::*;
use ndarray::{Array1, ArrayBase, Data, DataMut, Ix1};

impl<S: Data<Elem = T>, T: Clone> ToIter for ArrayBase<S, Ix1> {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = T>>
    where
        T: 'a,
    {
        TrustIter::new(self.iter().cloned(), self.len())
    }
}

impl<S: Data<Elem = T>, T: Clone> Vec1View for ArrayBase<S, Ix1> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.uget(index).clone()
    }
}

impl<'a, S: DataMut<Elem = T>, T: 'a + Clone> Vec1Mut<'a> for ArrayBase<S, Ix1> {
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        self.uget_mut(index)
    }
}

impl<T: Clone> Vec1 for Array1<T> {
    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        Array1::from_iter(iter)
    }

    #[inline]
    fn uninit<'a>(len: usize) -> impl UninitVec<'a, T, Vec = Self>
    where
        T: Copy + 'a,
    {
        Array1::uninit(len)
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self {
        let vec = iter.collect_trusted_to_vec();
        Array1::from_vec(vec)
    }
}

impl<'a, T: 'a + Copy> UninitVec<'a, T> for Array1<MaybeUninit<T>> {
    type Vec = Array1<T>;
    unsafe fn assume_init(self) -> Self::Vec {
        self.assume_init()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use ndarray::Array1;

    #[test]
    fn test_basic() {
        let data = Array1::from(vec![1, 2, 3, 4, 5]);
        let view = data.view();
        assert_eq!(ToIter::len(&data), 5);
        assert_eq!(Vec1View::get(&view, 0), 1);
    }

    #[test]
    fn test_get_mut() {
        let mut data = Array1::from(vec![1, 2, 3, 4, 5]);
        *Vec1Mut::get_mut(&mut data, 0).unwrap() = 10;
        assert_eq!(data.get(0), Some(&10));
        let mut view = data.view_mut();
        assert_eq!(Vec1Mut::get_mut(&mut view, 1), Some(&mut 2));
    }
}
