use std::mem::MaybeUninit;

use crate::prelude::*;

macro_rules! impl_vec1 {
    (to_iter $($ty: ty),*) => {
        $(impl<T: Clone> ToIter for $ty {
            type Item = T;

            #[inline]
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
        })*
    };

    (view $($({$N: ident})? $ty: ty),*) => {
        $(
            impl<T: Clone $(, const $N: usize)?> Vec1View for $ty {
                #[inline]
                unsafe fn uget(&self, index: usize) -> T {
                    self.get_unchecked(index).clone()
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
        )*
    };
}

impl<T: Clone, const N: usize> ToIter for [T; N] {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        T: 'a,
    {
        TrustIter::new(self.iter().cloned(), self.len())
    }
}

impl<T: Clone, const N: usize> ToIter for &[T; N] {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn to_iterator<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        T: 'a,
    {
        TrustIter::new(self.iter().cloned(), self.len())
    }
}

impl_vec1!(to_iter Vec<T>, &[T], &Vec<T>);
impl_vec1!(view Vec<T>, &[T], &Vec<T>, {N} &[T; N], {N} [T; N]);

impl<'a, T: Clone + 'a> Vec1Mut<'a> for Vec<T> {
    #[inline]
    unsafe fn uget_mut(&'a mut self, index: usize) -> &'a mut T {
        self.get_unchecked_mut(index)
    }
}

impl<T: Clone> Vec1 for Vec<T> {
    type Uninit = Vec<MaybeUninit<T>>;
    type UninitRefMut<'a> = &'a mut [MaybeUninit<T>] where T: 'a;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline]
    fn uninit(len: usize) -> Self::Uninit {
        let mut v = Vec::with_capacity(len);
        unsafe {
            v.set_len(len);
        }
        v
    }

    #[inline]
    fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_> {
        uninit_vec
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

impl<T: Clone> UninitVec<T> for Vec<MaybeUninit<T>> {
    type Vec = Vec<T>;

    #[inline]
    unsafe fn assume_init(self) -> Self::Vec {
        let (ptr, len, cap) = self.into_raw_parts();
        Vec::from_raw_parts(ptr as *mut T, len, cap)
    }

    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.get_unchecked_mut(idx);
        ele.write(v);
    }
}

impl<T> UninitRefMut<T> for &mut [MaybeUninit<T>] {
    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.get_unchecked_mut(idx);
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
        assert_eq!(ToIter::len(&[1, 2]), 2);
        assert_eq!(view.get(0), 1);
        assert_eq!([1, 2].get(0), 1);
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
        unsafe { data.uset(1, 2) };
        let data: Vec<_> = unsafe { data.assume_init() };
        assert_eq!(data, vec![1, 2]);
    }

    #[test]
    fn test_rolling_custom() {
        let data = vec![1, 2, 3, 4, 5];
        let out: Vec<_> = data.rolling_custom(3, |s| s.to_iter().vsum().unwrap());
        assert_eq!(out, vec![1, 3, 6, 9, 12]);
    }
}
