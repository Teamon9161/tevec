use std::mem::MaybeUninit;

use tea_deps::ndarray::{Array1, ArrayBase, ArrayView1, ArrayViewMut1, Data, Ix1};

use crate::prelude::*;

impl<S: Data<Elem = T>, T> GetLen for ArrayBase<S, Ix1> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Clone> TIter<T> for Array1<T> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = T>{
        self.iter().cloned()
    }

    #[inline]
    fn tditer(&self) -> impl TDoubleIterator<Item = T>{
        self.iter().cloned()
    }
}

impl<T: Clone> TIter<T> for ArrayView1<'_, T> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = T> {
        self.iter().cloned()
    }

    #[inline]
    fn tditer(&self) -> impl TDoubleIterator<Item = T>{
        self.iter().cloned()
    }
}

impl<T: Clone> TIter<T> for ArrayViewMut1<'_, T> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = T> {
        self.iter().cloned()
    }

    #[inline]
    fn tditer(&self) -> impl TDoubleIterator<Item = T>{
        self.iter().cloned()
    }
}

macro_rules! impl_vec1view_for_ndarray {
    ($t:ty $(, $lt:lifetime)?) => {
        impl<$($lt,)? T: Clone> Vec1View<T> for $t {
            type SliceOutput<'a> = ArrayView1<'a, T> where Self: 'a;

            #[inline]
            fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
            where
                T: 'a,
            {
                use tea_deps::ndarray::s;
                let view = self.slice(s![start..end]);
                Ok(view)
            }

            #[inline]
            fn get_backend_name(&self) -> &'static str {
                "ndarray"
            }

            #[inline]
            unsafe fn uget(&self, index: usize) -> T { unsafe {
                self.uget(index).clone()
            }}

            #[inline]
            fn try_as_slice(&self) -> Option<&[T]> {
                self.as_slice_memory_order()
            }

            #[inline]
            fn rolling_custom<'a, O: Vec1<OT>, OT: Clone, F>(
                &'a self,
                window: usize,
                f: F,
                out: Option<O::UninitRefMut<'_>>,
            ) -> Option<O>
            where
                F: FnMut(Self::SliceOutput<'a>) -> OT,
                T: 'a,
            {
                let len = self.len();
                if let Some(out) = out {
                    self.rolling_custom_to::<O, _, _>(window, f, out);
                    None
                } else {
                    use crate::prelude::UninitVec;
                    let mut out = O::uninit(len);
                    self.rolling_custom_to::<O, _, _>(window, f, O::uninit_ref_mut(&mut out));
                    Some(unsafe { out.assume_init() })
                }
            }

            #[inline]
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
                    self.rolling2_apply_idx_to::<O, _, _, _, _>(
                        other,
                        window,
                        f,
                        O::uninit_ref_mut(&mut out),
                    );
                    Some(unsafe { out.assume_init() })
                }
            }
        }
    };
}

impl_vec1view_for_ndarray!(Array1<T>);
impl_vec1view_for_ndarray!(ArrayView1<'t, T>, 't);
impl_vec1view_for_ndarray!(ArrayViewMut1<'t, T>, 't);

impl<'a, T: 'a + Clone> Vec1Mut<'a, T> for ArrayViewMut1<'a, T> {
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.uget_mut(index) }
    }

    #[inline]
    fn try_as_slice_mut(&mut self) -> Option<&mut [T]> {
        self.as_slice_mut()
    }
}

impl<T: Clone> Vec1Mut<'_, T> for Array1<T> {
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.uget_mut(index) }
    }

    #[inline]
    fn try_as_slice_mut(&mut self) -> Option<&mut [T]> {
        self.as_slice_mut()
    }
}

impl<T: Clone> Vec1<T> for Array1<T> {
    type Uninit = Array1<MaybeUninit<T>>;
    type UninitRefMut<'a>
        = ArrayViewMut1<'a, MaybeUninit<T>>
    where
        T: 'a;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        Array1::from_iter(iter)
    }

    #[inline]
    fn try_collect_from_iter<I: Iterator<Item = TResult<T>>>(iter: I) -> TResult<Self> {
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
    fn try_collect_from_trusted<I: Iterator<Item = TResult<T>> + TrustedLen>(
        iter: I,
    ) -> TResult<Self> {
        let vec = iter.try_collect_trusted_to_vec()?;
        Ok(Array1::from_vec(vec))
    }
}

impl<T: Clone> UninitVec<T> for Array1<MaybeUninit<T>> {
    type Vec = Array1<T>;
    #[inline]
    unsafe fn assume_init(self) -> Self::Vec {
        unsafe { self.assume_init() }
    }

    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        unsafe {
            let ele = self.uget_mut(idx);
            ele.write(v);
        }
    }
}

impl<T> UninitRefMut<T> for ArrayViewMut1<'_, MaybeUninit<T>> {
    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        unsafe {
            let ele = self.uget_mut(idx);
            ele.write(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use tea_deps::ndarray::Array1;

    use crate::prelude::*;

    #[test]
    fn test_basic() {
        let data = Array1::from(vec![1, 2, 3, 4, 5]);
        let view = data.view();
        assert_eq!(GetLen::len(&data), 5);
        assert_eq!(Vec1View::get(&view, 0).unwrap(), 1);
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
