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
        self.to_iter()
            .zip(remove_value_iter)
            .map(move |(v, v_remove)| f(v_remove, v))
            .collect_vec1()
    }

    fn rolling_apply_idx<O: Vec1, F>(&self, window: usize, mut f: F) -> O
    where
        O::Item: Clone,
        // start, end, value
        F: FnMut(Option<usize>, usize, T) -> O::Item,
    {
        assert!(window > 0, "window must be greater than 0");
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..self.len() - window + 1).map(Some));
        self.to_iter()
            .zip(start_iter)
            .enumerate()
            .map(move |(end, (v, start))| f(start, end, v))
            .collect_vec1()
    }
}

impl<T: Clone, I: Vec1View<Item = T>> RollingBasic<T> for I {}
