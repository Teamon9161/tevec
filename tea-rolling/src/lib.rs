#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
mod features;
use tea_core::prelude::*;
pub const EPS: f64 = 1e-14;

use std::iter::Iterator;

pub trait RollingBasic<T>: Vec1View<Item = T>
where
    T: Element,
{
    fn rolling_apply<'a, F, U: Element>(&'a self, window: usize, mut f: F) -> VecOutType<Self, U>
    //vec_out_type!(U)//<Self::Vec<U> as Vec1View<U>>::Vec<U> //Iterator<Item = U>
    where
        T: 'a,
        F: FnMut(Option<T>, T) -> U,
        Self::Vec<U>: Vec1<Item = U>,
    {
        assert!(window > 0, "window must be greater than 0");
        let remove_value_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain(self.to_iterator().map(|v| Some(v)));
        self.to_iterator()
            .zip(remove_value_iter)
            .map(move |(v, v_remove)| f(v_remove, v))
            .collect_vec1::<Self::Vec<U>>()
    }
}

pub trait RollingValidBasic<T>: Vec1View<Item = Option<T>>
where
    T: IsNone + Element,
{
    fn rolling_vapply<'a, F, U: Element>(&'a self, window: usize, mut f: F) -> VecOutType<Self, U>
    where
        T: 'a,
        F: FnMut(Option<Option<T>>, Option<T>) -> U,
        Self::Vec<U>: Vec1<Item = U>,
    {
        assert!(window > 0, "window must be greater than 0");
        let remove_value_iter = std::iter::repeat::<Option<Option<T>>>(None)
            .take(window - 1)
            .chain(self.to_iterator().map(Some));
        self.to_iterator()
            .zip(remove_value_iter)
            .map(move |(v, v_remove)| f(v_remove.map(|v| v.v()), v))
            .collect_vec1::<Self::Vec<U>>()
    }
}

impl<T: Element, I: Vec1View<Item = T>> RollingBasic<T> for I {}

impl<T: IsNone + Element, I: Vec1View<Item = Option<T>>> RollingValidBasic<T> for I {}
