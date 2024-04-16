use std::iter::Iterator;
use tea_core::prelude::*;

pub trait RollingBasic<T>: Vec1View<Item = T>
where
    T: Clone,
{
    fn rolling_apply<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        O::Item: Clone,
        F: FnMut(Option<T>, T) -> O::Item,
    {
        assert!(window > 0, "window must be greater than 0");
        let remove_value_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain(self.to_iterator().map(|v| Some(v)));
        self.to_iterator()
            .zip(remove_value_iter)
            .map(move |(v, v_remove)| f(v_remove, v))
            .collect_vec1()
    }
}

pub trait RollingValidBasic<T>: Vec1View<Item = Option<T>>
where
    T: IsNone + Clone,
{
    fn rolling_vapply<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        O::Item: Clone,
        F: FnMut(Option<Option<T>>, Option<T>) -> O::Item,
    {
        assert!(window > 0, "window must be greater than 0");
        let remove_value_iter = std::iter::repeat::<Option<Option<T>>>(None)
            .take(window - 1)
            .chain(self.to_iterator().map(Some));
        self.to_iterator()
            .zip(remove_value_iter)
            .map(move |(v, v_remove)| f(v_remove.map(|v| v.v()), v))
            .collect_vec1()
    }
}

impl<T: Clone, I: Vec1View<Item = T>> RollingBasic<T> for I {}

impl<T: IsNone + Clone, I: Vec1View<Item = Option<T>>> RollingValidBasic<T> for I {}
