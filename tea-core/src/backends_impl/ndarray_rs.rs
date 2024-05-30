use std::mem::MaybeUninit;

use crate::prelude::*;
use ndarray::{Array1, ArrayBase, ArrayViewMut1, Data, DataMut, Ix1};

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

    #[inline]
    fn try_as_slice(&self) -> Option<&[T]> {
        self.as_slice_memory_order()
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling_apply<O: Vec1, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Option<Self::Item>, Self::Item) -> O::Item,
    {
        let len = self.len();
        if let Some(out) = out {
            self.rolling_apply_to::<O, _>(window, f, out);
            None
        } else {
            let mut out = O::uninit(len);
            self.rolling_apply_to::<O, _>(window, f, O::uninit_ref_mut(&mut out));
            Some(unsafe { out.assume_init() })
        }
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling2_apply<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Option<(Self::Item, V2::Item)>, (Self::Item, V2::Item)) -> O::Item,
    {
        let len = self.len();
        if let Some(out) = out {
            self.rolling2_apply_to::<O, _, _>(other, window, f, out);
            None
        } else {
            let mut out = O::uninit(len);
            self.rolling2_apply_to::<O, _, _>(other, window, f, O::uninit_ref_mut(&mut out));
            Some(unsafe { out.assume_init() })
        }
    }

    #[inline]
    /// this should be a faster implemention than default as
    /// we read value directly by ptr
    fn rolling_apply_idx<O: Vec1, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Option<usize>, usize, Self::Item) -> O::Item,
    {
        let len = self.len();
        if let Some(out) = out {
            self.rolling_apply_idx_to::<O, _>(window, f, out);
            None
        } else {
            let mut out = O::uninit(len);
            self.rolling_apply_idx_to::<O, _>(window, f, O::uninit_ref_mut(&mut out));
            Some(unsafe { out.assume_init() })
        }
    }
}

impl<'a, S: DataMut<Elem = T>, T: 'a + Clone> Vec1Mut<'a> for ArrayBase<S, Ix1> {
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        self.uget_mut(index)
    }

    #[inline]
    fn try_as_slice_mut(&mut self) -> Option<&mut [Self::Item]> {
        self.as_slice_mut()
    }
}

impl<T: Clone> Vec1 for Array1<T> {
    type Uninit = Array1<MaybeUninit<T>>;
    type UninitRefMut<'a> = ArrayViewMut1<'a, MaybeUninit<T>> where T: 'a;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        Array1::from_iter(iter)
    }

    #[inline]
    fn try_collect_from_iter<I: Iterator<Item = TResult<Self::Item>>>(iter: I) -> TResult<Self> {
        let vec = iter.collect::<TResult<Vec<_>>>()?;
        Ok(Array1::from_vec(vec))
    }

    #[inline]
    fn uninit(len: usize) -> Self::Uninit {
        Array1::uninit(len)
    }

    #[inline]
    fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_> {
        uninit_vec.view_mut()
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self {
        let vec = iter.collect_trusted_to_vec();
        Array1::from_vec(vec)
    }

    #[inline]
    fn try_collect_from_trusted<I: Iterator<Item = TResult<Self::Item>> + TrustedLen>(
        iter: I,
    ) -> TResult<Self>
    where
        Self::Item: std::fmt::Debug,
    {
        let vec = iter.try_collect_trusted_to_vec()?;
        Ok(Array1::from_vec(vec))
    }
}

impl<T: Clone> UninitVec<T> for Array1<MaybeUninit<T>> {
    type Vec = Array1<T>;
    #[inline]
    unsafe fn assume_init(self) -> Self::Vec {
        self.assume_init()
    }

    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.uget_mut(idx);
        ele.write(v);
    }
}

impl<'a, T> UninitRefMut<T> for ArrayViewMut1<'a, MaybeUninit<T>> {
    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.uget_mut(idx);
        ele.write(v);
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
