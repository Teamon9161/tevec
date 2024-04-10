use ndarray::{Array1, ArrayBase, Data, DataMut, Ix1};

use super::super::{Vec1D, VecView1D, VecMut1D, TrustedLen, trusted::CollectTrustedToVec};
use crate::prelude::IsNone;

impl<S, T> VecView1D<T> for ArrayBase<S, Ix1> 
where
    S: Data<Elem = T>,
{   
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> &T {
        self.uget(index)
    }

    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<&T> 
    where
        T: IsNone
    {
        let v = self.uget(index);
        if v.is_none() {
            None
        } else {
            Some(v)
        }
    }
}

impl<S, T> VecMut1D<T> for ArrayBase<S, Ix1>
where
    S: DataMut<Elem = T>,
{   
    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        self.uget_mut(index)
    }
}

impl<T> Vec1D<T> for Array1<T>
{   
    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        Array1::from_iter(iter)
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T>+TrustedLen>(iter: I) -> Self 
    where Self: Sized 
    {
        let vec: Vec<T> = iter.collect_trusted_to_vec();
        Array1::from(vec)
    }
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;
    use crate::prelude::*;

    #[test]
    fn test_basic() {
        let data = Array1::from(vec![1, 2, 3, 4, 5]);
        let view = data.view();
        assert_eq!(VecView1D::len(&data), 5);
        assert_eq!(VecView1D::get(&data, 0), &1);
        let sum = VecView1D::iter_view(&view).fold(0, |acc, x| acc + *x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_iter() {
        let mut data = Array1::from(vec![1, 2, 3, 4, 5]);
        let view = data.view();
        let mut view_iter = VecView1D::iter_view(&view);
        assert_eq!(view_iter.next_back(), Some(&5));
        assert_eq!(view_iter.next_back(), Some(&4));
        assert_eq!(view_iter.next_back(), Some(&3));
        assert_eq!(view_iter.next_back(), Some(&2));
        assert_eq!(view_iter.next_back(), Some(&1));
        assert_eq!(view_iter.next_back(), None);
        VecMut1D::iter_mut(&mut data).for_each(|x| *x += 1);
        assert_eq!(data.get(4), Some(&6));
        let data = Array1::from(vec![1, 2, 3, 4, 5]);
        let mut iter = Vec1D::into_iter(data);
        assert_eq!(iter.next(), Some(1));
        assert!(iter.fold(0, |acc, x| acc + x) == 14);
    }

    #[test]
    fn test_collect() {
        let data: Array1<_> = (0..5).collect_vec1d();
        assert_eq!(data, Array1::from(vec![0, 1, 2, 3, 4]));
        let data: Array1<_> = (0..5).collect_trusted();
        assert_eq!(data, Array1::from(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_map() {
        let data = Array1::from(vec![1, 2, 3, 4, 5]);
        let new_data: Array1<_> = VecView1D::map(&data, |x| x + 1);
        assert_eq!(new_data, Array1::from(vec![2, 3, 4, 5, 6]));
    }
}