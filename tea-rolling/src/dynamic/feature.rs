use tea_core::prelude::*;
use tea_dyn::{DynVec1, IntoDyn};

use crate::*;

pub trait RollingFeatureDyn: DynVec1 + Sized {
    /// Calculates the rolling mean of valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    /// * `out` - Optional output buffer.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling means.
    fn ts_mean(&self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vmean::<Self::F64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vmean::<Self::F32Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vmean::<Self::I64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vmean::<Self::I32Vec, _>(window, min_periods)
                .into_dyn(),
            dtype => tbail!("Unsupported dtype: {:?} in ts_mean", dtype),
        };
        if let Some(name) = name {
            Ok(res.with_name(name))
        } else {
            Ok(res)
        }
    }

    /// Calculates the exponentially weighted moving average.
    ///
    /// # Arguments
    /// * `window` - The size of the moving window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    ///
    /// # Returns
    /// A new Series with the calculated values.
    fn ts_ewm(&self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vewm::<Self::F64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vewm::<Self::F32Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vewm::<Self::I64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vewm::<Self::I32Vec, _>(window, min_periods)
                .into_dyn(),
            dtype => tbail!("Unsupported dtype: {:?} in ts_ewm", dtype),
        };
        if let Some(name) = name {
            Ok(res.with_name(name))
        } else {
            Ok(res)
        }
    }

    /// Calculates the rolling standard deviation of valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    ///
    /// # Returns
    ///
    /// A vector containing the rolling standard deviations.
    fn ts_std(&self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vstd::<Self::F64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vstd::<Self::F32Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vstd::<Self::I64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vstd::<Self::I32Vec, _>(window, min_periods)
                .into_dyn(),
            dtype => tbail!("Unsupported dtype: {:?} in ts_std", dtype),
        };
        if let Some(name) = name {
            Ok(res.with_name(name))
        } else {
            Ok(res)
        }
    }

    /// Calculates the rolling skewness for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    ///
    /// # Returns
    ///
    /// A new Series with the calculated rolling skewness values.
    fn ts_skew(&self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vskew::<Self::F64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vskew::<Self::F32Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vskew::<Self::I64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vskew::<Self::I32Vec, _>(window, min_periods)
                .into_dyn(),
            dtype => tbail!("Unsupported dtype: {:?} in ts_skew", dtype),
        };
        if let Some(name) = name {
            Ok(res.with_name(name))
        } else {
            Ok(res)
        }
    }

    /// Calculates the rolling kurtosis for valid elements within a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The size of the rolling window.
    /// * `min_periods` - The minimum number of observations in window required to have a value.
    ///
    /// # Returns
    ///
    /// A new Series with the calculated rolling kurtosis values.
    fn ts_kurt(&self, window: usize, min_periods: Option<usize>) -> TResult<Self>
    where
        f64: Cast<Self::F64Item> + Cast<Self::F32Item> + Cast<Self::I64Item> + Cast<Self::I32Item>,
    {
        let name = self.get_name();
        let res = match self.get_dtype() {
            DataType::F64 => self
                .extract_f64()
                .unwrap()
                .ts_vkurt::<Self::F64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::F32 => self
                .extract_f32()
                .unwrap()
                .ts_vkurt::<Self::F32Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I64 => self
                .extract_i64()
                .unwrap()
                .ts_vkurt::<Self::I64Vec, _>(window, min_periods)
                .into_dyn(),
            DataType::I32 => self
                .extract_i32()
                .unwrap()
                .ts_vkurt::<Self::I32Vec, _>(window, min_periods)
                .into_dyn(),
            dtype => tbail!("Unsupported dtype: {:?} in ts_kurt", dtype),
        };
        if let Some(name) = name {
            Ok(res.with_name(name))
        } else {
            Ok(res)
        }
    }
}

impl<S: DynVec1> RollingFeatureDyn for S {}

#[cfg(test)]
mod tests {
    use tea_deps::polars::prelude::*;

    use super::*;

    #[test]
    fn test_ts_mean() {
        let series = Series::from_vec("abc".into(), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = series.ts_mean(3, None).unwrap();

        let expected = Series::from_vec("abc".into(), vec![1., 1.5, 2.0, 3.0, 4.0]);

        assert_eq!(result, expected);

        // Test with min_periods
        let result_min_periods = series.ts_mean(3, Some(3)).unwrap();
        let expected_min_periods: Float64Chunked =
            vec![None, None, Some(2.0), Some(3.0), Some(4.0)].collect_trusted_vec1();

        assert_eq!(result_min_periods, expected_min_periods.into_dyn());
    }
}
