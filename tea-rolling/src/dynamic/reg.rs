use tea_core::prelude::*;
use tea_dyn::{DynVec1, IntoDyn};

use crate::*;

pub trait RollingRegDyn: DynVec1 + Sized {
    fn ts_regx_beta(&self, x: Self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vregx_beta::<Self::F64Vec, _, _, _>(
                    &x.cast::<f64>()?.extract_f64().unwrap(),
                    window,
                    min_periods,
                )
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vregx_beta::<Self::F32Vec, _, _, _>(
                    &x.cast::<f32>()?.extract_f32().unwrap(),
                    window,
                    min_periods,
                )
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vregx_beta::<Self::I64Vec, _, _, _>(
                    &x.cast::<i64>()?.extract_i64().unwrap(),
                    window,
                    min_periods,
                )
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vregx_beta::<Self::I32Vec, _, _, _>(
                    &x.cast::<i32>()?.extract_i32().unwrap(),
                    window,
                    min_periods,
                )
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

impl<T: DynVec1> RollingRegDyn for T {}
