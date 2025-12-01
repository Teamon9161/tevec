use std::collections::VecDeque;
use std::mem::MaybeUninit;

use crate::prelude::*;

impl<T> GetLen for VecDeque<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Clone> TIter<T> for VecDeque<T> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = T> {
        self.iter().cloned()
    }

    #[inline]
    fn tditer(&self) -> impl TDoubleIterator<Item = T> {
        self.iter().cloned()
    }
}

impl<T: Clone> Vec1View<T> for VecDeque<T> {
    type SliceOutput<'a>
        = std::collections::vec_deque::Iter<'a, T>
    where
        Self: 'a;

    #[inline]
    fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
        Ok(self.range(start..end))
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        "vecdeque"
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.get(index).unwrap().clone()
    }

    #[inline]
    fn try_as_slice(&self) -> Option<&[T]> {
        let slc = self.as_slices();
        if slc.1.is_empty() { Some(slc.0) } else { None }
    }
}

impl<T: Clone> Vec1Mut<'_, T> for VecDeque<T> {
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }

    #[inline]
    fn try_as_slice_mut(&mut self) -> Option<&mut [T]> {
        let slc = self.as_mut_slices();
        if slc.1.is_empty() { Some(slc.0) } else { None }
    }
}

impl<T: Clone> Vec1<T> for VecDeque<T> {
    type Uninit = VecDeque<MaybeUninit<T>>;
    type UninitRefMut<'a>
        = &'a mut VecDeque<MaybeUninit<T>>
    where
        T: 'a;

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
        let v = Vec::uninit(len);
        VecDeque::from(v)
    }

    #[inline]
    fn uninit_ref_mut<'a>(uninit_vec: &'a mut Self::Uninit) -> Self::UninitRefMut<'a>
    where
        T: 'a,
    {
        uninit_vec
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self {
        iter.collect_trusted_to_vec().into()
    }

    #[inline]
    fn try_collect_from_trusted<I: Iterator<Item = TResult<T>> + TrustedLen>(
        iter: I,
    ) -> TResult<Self> {
        let v: Vec<_> = iter.try_collect_trusted_to_vec()?;
        Ok(v.into())
    }

    #[inline]
    fn empty() -> Self {
        VecDeque::new()
    }
}

impl<T: Clone> UninitVec<T> for VecDeque<MaybeUninit<T>> {
    type Vec = VecDeque<T>;

    #[inline]
    unsafe fn assume_init(self) -> Self::Vec {
        self.into_iter()
            .map(|x| unsafe { x.assume_init() })
            .collect()
    }

    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.get_mut(idx).unwrap();
        ele.write(v);
    }
}

impl<T> UninitRefMut<T> for &mut VecDeque<MaybeUninit<T>> {
    #[inline]
    unsafe fn uset(&mut self, idx: usize, v: T) {
        let ele = self.get_mut(idx).unwrap();
        ele.write(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecdeque_basic() {
        let data: VecDeque<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(GetLen::len(&data), 5);
        assert_eq!(Vec1View::get(&data, 0).unwrap(), 1);
        assert_eq!(Vec1View::get(&data, 4).unwrap(), 5);
    }

    #[test]
    fn test_vecdeque_slice() {
        let data: VecDeque<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        let slice = data.slice(1, 4).unwrap().cloned().collect_trusted_to_vec();
        assert_eq!(slice, vec![2, 3, 4]);
    }

    #[test]
    fn test_vecdeque_mut() {
        let mut data: VecDeque<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        *Vec1Mut::get_mut(&mut data, 0).unwrap() = 10;
        assert_eq!(data[0], 10);
    }

    #[test]
    fn test_vecdeque_collect() {
        let data: VecDeque<i32> = Vec1::collect_from_iter(1..6);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_vecdeque_uninit() {
        let mut uninit = VecDeque::uninit(3);
        unsafe {
            UninitVec::uset(&mut uninit, 0, 1);
            UninitVec::uset(&mut uninit, 1, 2);
            UninitVec::uset(&mut uninit, 2, 3);
            let init: VecDeque<i32> = uninit.assume_init();
            assert_eq!(init, vec![1, 2, 3]);
        }
    }

    #[test]
    fn test_vecdeque_get() {
        let data: VecDeque<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(Vec1View::get(&data, 0).unwrap(), 1);
        assert_eq!(Vec1View::get(&data, 4).unwrap(), 5);
    }

    #[test]
    fn test_vecdeque_rolling_custom() {
        let data: VecDeque<i32> = (1..5).collect();
        let window_size = 3;

        // Define a custom function to calculate the sum of each window
        let sum_window =
            |slice: std::collections::vec_deque::Iter<'_, i32>| slice.cloned().vsum().unwrap();

        // Apply rolling_custom
        let result: VecDeque<i32> = data.rolling_custom(window_size, sum_window, None).unwrap();
        assert_eq!(result, vec![1, 3, 6, 9]);
    }
}
