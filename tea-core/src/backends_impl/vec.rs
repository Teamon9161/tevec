use crate::prelude::*;

impl<T: Clone> ToIter for Vec<T> {
    type Item = T;
    #[inline]
    fn to_iterator<'a>(&'a self) -> impl Iterator<Item = T>
    where
        T: 'a,
    {
        self.iter().cloned()
    }
}

impl<T: Clone> ToIter for &[T] {
    type Item = T;
    #[inline]
    fn to_iterator<'a>(&'a self) -> impl Iterator<Item = T>
    where
        T: 'a,
    {
        self.iter().cloned()
    }
}

impl<T: Clone> Vec1View for Vec<T> {
    // type Vec<U: Element> = Vec<U>;
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.get_unchecked(index).clone()
    }
}

impl<T: Clone> Vec1View for &[T] {
    // type Vec<U: Element> = Vec<U>;
    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.get_unchecked(index).clone()
    }
}

impl<'a, T: Clone + 'a> Vec1Mut<'a> for Vec<T> {
    #[inline]
    unsafe fn uget_mut(&'a mut self, index: usize) -> &'a mut T {
        self.get_unchecked_mut(index)
    }
}

impl<T: Clone> Vec1 for Vec<T> {
    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self {
        iter.collect_trusted_to_vec()
    }

    #[inline]
    fn empty() -> Self {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_get() {
        let data = vec![1, 2, 3, 4, 5];
        let view = &data;
        assert_eq!(Vec1View::len(&data), 5);
        assert_eq!(view.get(0), 1);
        let slice = view.as_slice();
        assert_eq!(unsafe { slice.uget(2) }, 3);
    }

    #[test]
    fn test_get_mut() {
        let mut data = vec![1, 2, 3];
        *unsafe { Vec1Mut::uget_mut(&mut data, 1) } = 4;
        assert_eq!(data[1], 4);
        let mut_ref = &mut data;
        *unsafe { Vec1Mut::uget_mut(mut_ref, 1) } = 4;
    }

    #[test]
    fn test_collect() {
        let data = (0..5).collect_vec1::<Vec<_>>();
        assert_eq!(data, vec![0, 1, 2, 3, 4]);
        let data = (0..5).collect_trusted_vec1::<Vec<_>>();
        assert_eq!(data, vec![0, 1, 2, 3, 4]);
        let v: Vec<i32> = vec![];
        let data: Vec<i32> = Vec::empty();
        assert_eq!(data, v);
        let data = vec![Some(1.), None, Some(2.)].collect_vec1_opt::<Vec<f64>>();
        assert!(data[1].is_nan());
        assert_eq!(data[2], 2.)
    }
}
