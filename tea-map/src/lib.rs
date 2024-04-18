#![feature(return_position_impl_trait_in_trait)]
use num_traits::Signed;
use std::iter::Iterator;
use tea_core::prelude::*;

pub trait MapBasic<T>: ToIter<Item = T>
where
    T: Clone,
{
    #[inline]
    fn abs(&self) -> impl Iterator<Item = T>
    where
        T: Signed,
    {
        self.map(|v| v.abs())
    }
}

pub trait MapValidBasic<T>: Vec1View<Item = Option<T>>
where
    T: IsNone + Clone,
{
}

impl<T: Clone, I: Vec1View<Item = T>> MapBasic<T> for I {}

impl<T: IsNone + Clone, I: Vec1View<Item = Option<T>>> MapValidBasic<T> for I {}
