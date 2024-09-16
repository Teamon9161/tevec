use tea_core::prelude::*;
use tea_dyn::{DynVec1, IntoDyn};

use crate::*;

pub trait RollingNormDyn: DynVec1 + Sized {
    fn ts_zscore(&self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vzscore::<Self::F64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vzscore::<Self::F32Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vzscore::<Self::I64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vzscore::<Self::I32Vec, _>(window, min_periods)
                .into_dyn(),
            dtype => tbail!("Unsupported dtype: {:?} in ts_zscore", dtype),
        };
        if let Some(name) = name {
            Ok(res.with_name(name))
        } else {
            Ok(res)
        }
    }
}

impl<T: DynVec1> RollingNormDyn for T {}
