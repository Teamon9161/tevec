use super::{RollingBasic, RollingValidBasic};
use tea_core::prelude::*;

pub trait RollingValidCmp<T: IsNone + Clone>: RollingValidBasic<T> {
    fn ts_vargmin<O: Vec1<Item = Option<T>>>(&self, window: usize, min_periods: Option<usize>) -> O
    where
        T: Number,
    {
        let window = min(self.len(), window);
        let mut min: T = T::max_();
        let mut min_idx: usize = 0;
        let mut n = 0;
        self.rolling_vapply_idx(window, min_periods, |start, end, v| {
            
        })
    }
}