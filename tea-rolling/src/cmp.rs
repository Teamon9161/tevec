use super::{RollingBasic, RollingValidBasic};
use tea_core::prelude::*;

pub trait RollingValidCmp<T: IsNone + Clone>: RollingValidBasic<T> {
    fn ts_vsum<O: Vec1<Item = Option<T>>>(&self, window: usize, min_periods: Option<usize>) -> O
    where
        T: Number,
    {
        let window = min(self.len(), window);
        
    }
}