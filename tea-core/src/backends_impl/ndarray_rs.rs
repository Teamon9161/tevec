use crate::{prelude::*, vec_core::Element};
use ndarray::{Array1, ArrayBase, Data, DataMut, Ix1};

impl<S: Data<Elem = T>, T: Clone> ToIter for ArrayBase<S, Ix1> {
    type Item = T;
    #[inline]
    fn to_iterator<'a>(&'a self) -> impl Iterator<Item = T>
    where
        T: 'a,
    {
        self.iter().cloned()
    }
}

impl<S: Data<Elem = T>, T: Clone> Vec1View for ArrayBase<S, Ix1> {
    type Vec<U: Element> = Array1<U>;
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.uget(index).clone()
    }
}

impl<'a, S: DataMut<Elem = T>, T: 'a + Clone> Vec1Mut<'a> for ArrayBase<S, Ix1> {
    #[inline]
    unsafe fn uget_mut(&'a mut self, index: usize) -> &'a mut T {
        self.uget_mut(index)
    }
}

impl<T: Element> Vec1 for Array1<T> {
    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        Array1::from_iter(iter)
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self {
        let vec = iter.collect_trusted_to_vec();
        Array1::from_vec(vec)
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
        assert_eq!(Vec1View::len(&data), 5);
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
