use std::mem::MaybeUninit;

use crate::prelude::*;

macro_rules! impl_vec1 {
    (to_iter $($ty: ty),* $(,)?) => {
        $(
            impl<T> GetLen for $ty {
                #[inline]
                fn len(&self) -> usize {
                    (*self).len()
                }
            }

            impl<T: Clone> TIter<T> for $ty {
                // type Item = T;

                #[inline]
                fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = T>>
                where
                    T: 'a,
                {
                    TrustIter::new(self.iter().cloned(), self.len())
                }
            }
        )*
    };

    (view $($({$N: ident})? $(--$slice: ident)? $ty: ty),* $(,)?) => {
        $(
            impl<T: Clone $(, const $N: usize)?> Slice<T> for $ty {
                // type Element = T;
                type Output<'a> = <Self as std::ops::Index<std::ops::Range<usize>>>::Output
                where T: 'a;
                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<std::borrow::Cow<'a, Self::Output<'a>>>
                where T: 'a
                {
                    use std::ops::Index;
                    Ok(std::borrow::Cow::Borrowed(self.index(start..end)))
                }
            }

            impl<T: Clone $(, const $N: usize)?> Vec1View<T> for $ty {
                #[inline]
                unsafe fn uget(&self, index: usize) -> T {
                    self.get_unchecked(index).clone()
                }

                $(#[inline]
                fn $slice(&self) -> Option<&[T]> {
                    Some(self)
                })?

                #[inline]
                /// this should be a faster implemention than default as
                /// we read value directly by ptr
                fn rolling_apply<O: Vec1<OT>, OT, F>(
                    &self,
                    window: usize,
                    f: F,
                    out: Option<O::UninitRefMut<'_>>,
                ) -> Option<O>
                where
                    F: FnMut(Option<T>, T) -> OT,
                {
                    let len = self.len();
                    if let Some(out) = out {
                        self.rolling_apply_to::<O, _, _>(window, f, out);
                        None
                    } else {
                        let mut out = O::uninit(len);
                        self.rolling_apply_to::<O, _, _>(window, f, O::uninit_ref_mut(&mut out));
                        Some(unsafe { out.assume_init() })
                    }
                }

                #[inline]
                /// this should be a faster implemention than default as
                /// we read value directly by ptr
                fn rolling2_apply<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
                    &self,
                    other: &V2,
                    window: usize,
                    f: F,
                    out: Option<O::UninitRefMut<'_>>,
                ) -> Option<O>
                where
                    F: FnMut(Option<(T, T2)>, (T, T2)) -> OT,
                {
                    let len = self.len();
                    if let Some(out) = out {
                        self.rolling2_apply_to::<O, _, _, _, _>(other, window, f, out);
                        None
                    } else {
                        let mut out = O::uninit(len);
                        self.rolling2_apply_to::<O, _, _, _, _>(other, window, f, O::uninit_ref_mut(&mut out));
                        Some(unsafe { out.assume_init() })
                    }
                }

                #[inline]
                /// this should be a faster implemention than default as
                /// we read value directly by ptr
                fn rolling_apply_idx<O: Vec1<OT>, OT, F>(
                    &self,
                    window: usize,
                    f: F,
                    out: Option<O::UninitRefMut<'_>>,
                ) -> Option<O>
                where
                    F: FnMut(Option<usize>, usize, T) -> OT,
                {
                    let len = self.len();
                    if let Some(out) = out {
                        self.rolling_apply_idx_to::<O, _, _>(window, f, out);
                        None
                    } else {
                        let mut out = O::uninit(len);
                        self.rolling_apply_idx_to::<O, _, _>(window, f, O::uninit_ref_mut(&mut out));
                        Some(unsafe { out.assume_init() })
                    }
                }

                #[inline]
                /// this should be a faster implemention than default as
                /// we read value directly by ptr
                fn rolling2_apply_idx<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
                    &self,
                    other: &V2,
                    window: usize,
                    f: F,
                    out: Option<O::UninitRefMut<'_>>,
                ) -> Option<O>
                where
                F: FnMut(Option<usize>, usize, (T, T2)) -> OT,
                {
                    let len = self.len();
                    if let Some(out) = out {
                        self.rolling2_apply_idx_to::<O, _, _, _, _>(other, window, f, out);
                        None
                    } else {
                        let mut out = O::uninit(len);
                        self.rolling2_apply_idx_to::<O, _, _, _, _>(other, window, f, O::uninit_ref_mut(&mut out));
                        Some(unsafe { out.assume_init() })
                    }
                }
            }
        )*
    };
}

impl<T, const N: usize> GetLen for [T; N] {
    #[inline]
    fn len(&self) -> usize {
        N
    }
}

impl<T: Clone, const N: usize> TIter<T> for [T; N] {
    // type Item = T;

    #[inline]
    fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = T>>
    where
        T: 'a,
    {
        TrustIter::new(self.iter().cloned(), self.len())
    }
}

impl_vec1!(
    to_iter
    Vec<T>,
    [T],
    // &[T],
);

// impl<T: Clone> IntoTIter for Vec<T> {
//     #[inline]
//     fn into_titer(self) -> TrustIter<Self::IntoIter> {
//         let len = self.len();
//         self.into_iter().to_trust(len)
//     }
// }

// impl<'a, T> IntoTIter for &'a [T] {
//     fn into_titer(self) -> TrustIter<Self::IntoIter> {
//         let len = self.len();
//         self.into_iter().to_trust(len)
//     }
// }

// impl<'a, T> IntoTIter for &'a mut [T] {
//     fn into_titer(self) -> TrustIter<Self::IntoIter> {
//         let len = self.len();
//         self.into_iter().to_trust(len)
//     }
// }

impl_vec1!(
    view
    --try_as_slice Vec<T>,
    --try_as_slice [T],
    // --try_as_slice &[T],
    {N} [T; N]
);

impl<'a, T: Clone + 'a> Vec1Mut<'a, T> for Vec<T> {
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        self.get_unchecked_mut(index)
    }

    #[inline]
    fn try_as_slice_mut(&mut self) -> Option<&mut [T]> {
        Some(self.as_mut_slice())
    }
}

impl<T: Clone> Vec1<T> for Vec<T> {
    type Uninit = Vec<MaybeUninit<T>>;
    type UninitRefMut<'a> = &'a mut [MaybeUninit<T>] where T: 'a;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline]
    fn try_collect_from_iter<I: Iterator<Item = TResult<T>>>(iter: I) -> TResult<Self> {
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
    fn try_collect_from_trusted<I: Iterator<Item = TResult<T>> + TrustedLen>(
        iter: I,
    ) -> TResult<Self>
    where
        T: std::fmt::Debug,
    {
        iter.try_collect_trusted_to_vec()
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
        std::mem::transmute::<Vec<MaybeUninit<T>>, Vec<T>>(self)
    }

    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.get_unchecked_mut(idx);
        ele.write(v);
    }
}

impl<T> GetLen for &mut [MaybeUninit<T>] {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
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
        let mut data = vec![1, 2, 3, 4, 5];
        {
            let view_mut: &mut [_] = &mut data;
            assert_eq!(GetLen::len(view_mut), 5);
            assert_eq!(Vec1View::get(view_mut, 2).unwrap(), 3);
        }
        let view = &data;
        assert_eq!(GetLen::len(&data), 5);
        assert_eq!(GetLen::len(&[1, 2]), 2);
        assert_eq!(view.get(0).unwrap(), 1);
        assert_eq!([1, 2].get(0).unwrap(), 1);
        let slice = view.as_slice();
        assert_eq!(unsafe { slice.uget(2) }, 3);
    }

    // #[test]
    // fn test_slice() {
    //     // use super::CollectTrustedToVec;
    //     let v = vec![1, 2, 4, 5, 2];
    //     let res = v.slice(0, 3).unwrap().titer().collect_trusted_to_vec();
    //     assert_eq!(&res, &[1, 2, 4]);
    //     let res = v.slice(2, 4).unwrap().titer().collect_trusted_to_vec();
    //     assert_eq!(&res, &[4, 5, 2]);
    // }

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
        assert_eq!(data[2], 2.);
        let data = vec![Ok(1), Ok(2), Err(terr!("err")), Ok(3)];
        let v: TResult<Vec<_>> = data.try_collect_vec1();
        assert!(v.is_err());
        let data = vec![Ok(1), Ok(2), Ok(3)];
        let v: TResult<Vec<_>> = data.try_collect_trusted_vec1();
        assert_eq!(v.unwrap(), vec![1, 2, 3]);
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
        let out: Vec<_> = data
            .rolling_custom(3, |s| s.titer().vsum().unwrap(), None)
            .unwrap();
        assert_eq!(out, vec![1, 3, 6, 9, 12]);
    }
}
