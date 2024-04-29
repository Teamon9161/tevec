use std::mem::MaybeUninit;

use crate::prelude::*;

impl<T: Clone> ToIter for Vec<T> {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        T: 'a,
    {
        TrustIter::new(self.iter().cloned(), self.len())
    }
}

impl<T: Clone> ToIter for &[T] {
    type Item = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        T: 'a,
    {
        TrustIter::new(self.iter().cloned(), self.len())
    }
}

impl<T: Clone> Vec1View for Vec<T> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.get_unchecked(index).clone()
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling_apply<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        F: FnMut(Option<Self::Item>, Self::Item) -> O::Item,
    {
        assert!(window > 0, "window must be greater than 0");
        let len = self.len();
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..len).map(Some)); // this is longer than expect, but start_iter will stop earlier
        start_iter
            .zip(0..len)
            .map(|(start, end)| {
                let v_remove = start.map(|v| unsafe { self.uget(v) });
                let v = unsafe { self.uget(end) };
                f(v_remove, v)
            })
            .collect_trusted_vec1()
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling_apply_idx<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        F: FnMut(Option<usize>, usize, Self::Item) -> O::Item,
    {
        assert!(window > 0, "window must be greater than 0");
        let len = self.len();
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..len).map(Some)); // this is longer than expect, but start_iter will stop earlier
        start_iter
            .zip(0..len)
            .map(|(start, end)| {
                let v = unsafe { self.uget(end) };
                f(start, end, v)
            })
            .collect_trusted_vec1()
    }
}

impl<T: Clone> Vec1View for &[T] {
    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.get_unchecked(index).clone()
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling_apply<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        F: FnMut(Option<Self::Item>, Self::Item) -> O::Item,
    {
        let len = self.len();
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..len).map(Some)); // this is longer than expect, but start_iter will stop earlier
        start_iter
            .zip(0..len)
            .map(|(start, end)| {
                let v_remove = start.map(|v| unsafe { self.uget(v) });
                let v = unsafe { self.uget(end) };
                f(v_remove, v)
            })
            .collect_trusted_vec1()
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling_apply_idx<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        F: FnMut(Option<usize>, usize, Self::Item) -> O::Item,
    {
        assert!(window > 0, "window must be greater than 0");
        let len = self.len();
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..len).map(Some)); // this is longer than expect, but start_iter will stop earlier
        start_iter
            .zip(0..len)
            .map(|(start, end)| {
                let v = unsafe { self.uget(end) };
                f(start, end, v)
            })
            .collect_trusted_vec1()
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
    fn uninit<'a>(len: usize) -> impl UninitVec<'a, T, Vec = Self>
    where
        T: Copy + 'a,
    {
        let mut v = Vec::with_capacity(len);
        unsafe {
            v.set_len(len);
        }
        v
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

impl<'a, T: 'a + Copy> UninitVec<'a, T> for Vec<MaybeUninit<T>> {
    type Vec = Vec<T>;

    #[inline]
    unsafe fn assume_init(self) -> Self::Vec {
        let (ptr, len, cap) = self.into_raw_parts();
        Vec::from_raw_parts(ptr as *mut T, len, cap)
    }

    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.uget_mut(idx);
        ele.write(v);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_get() {
        let data = vec![1, 2, 3, 4, 5];
        let view = &data;
        assert_eq!(ToIter::len(&data), 5);
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

    #[test]
    fn test_iter_cast() {
        let data = vec![1, 2, 3, 4, 5];
        let out: Vec<_> = data.iter_cast::<f64>().collect_trusted_vec1();
        assert_eq!(out, vec![1., 2., 3., 4., 5.]);
        let data = vec![Some(1), Some(2), None];
        let out: Vec<_> = data.opt_iter_cast::<f64>().collect_vec1();
        assert_eq!(out, vec![Some(1.), Some(2.), None])
    }

    #[test]
    fn test_uninit() {
        let mut data = Vec::<i32>::uninit(2);
        unsafe { data.uset(0, 1) };
        data.set(1, 2);
        let data: Vec<_> = unsafe { data.assume_init() };
        assert_eq!(data, vec![1, 2]);
    }
}
