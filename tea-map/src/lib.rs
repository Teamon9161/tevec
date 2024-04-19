#![feature(return_position_impl_trait_in_trait)]
use num_traits::Signed;
use tea_core::prelude::*;

pub trait MapBasic<T>: ToIter<Item = T>
where
    T: Clone,
{
    #[inline]
    fn abs<O: Vec1<Item = T>>(&self) -> O
    where
        T: Signed,
    {
        self.map(|v| v.abs()).collect_trusted_vec1()
    }
}

pub trait MapValidBasic<T>: ToIter<Item = Option<T>>
where
    T: Clone,
{
    #[inline]
    fn vabs<O: Vec1<Item = Option<T>>>(&self) -> O
    where
        T: Signed,
    {
        self.map(|v| v.map(|v| v.abs())).collect_trusted_vec1()
    }
}

impl<T: Clone, I: Vec1View<Item = T>> MapBasic<T> for I {}

impl<T: Clone, I: Vec1View<Item = Option<T>>> MapValidBasic<T> for I {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_abs() {
        let v = vec![1, -2, 3, -4, 5];
        let res: Vec<_> = v.abs();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
        let v = vec![Some(1), Some(-2), None, Some(-4), Some(5)];
        let res: Vec<_> = v.vabs();
        assert_eq!(res, vec![Some(1), Some(2), None, Some(4), Some(5)]);
    }
}
