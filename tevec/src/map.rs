pub use tea_map::*;

use tea_core::prelude::*;

#[cfg(feature = "agg")]
use tea_agg::*;

#[cfg(feature = "agg")]
#[derive(Copy, Clone)]
pub enum WinsorizeMethod {
    Quantile,
    Median,
    Sigma,
}

pub trait MapValidFinal<T: IsNone>: Vec1View<Item = T> {
    #[cfg(feature = "agg")]
    fn winsorize<'a>(
        &'a self,
        method: WinsorizeMethod,
        method_params: Option<f64>,
    ) -> TResult<Box<dyn TrustedLen<Item = f64> + 'a>>
    where
        T: Cast<f64>,
        T::Inner: Number,
        Self: VecAggValidExt<T>,
    {
        use WinsorizeMethod::*;
        match method {
            Quantile => {
                use tea_agg::QuantileMethod;
                let method_params = method_params.unwrap_or(0.01);
                let min = self.vquantile(method_params, QuantileMethod::Linear)?;
                let max = self.vquantile(1. - method_params, QuantileMethod::Linear)?;
                Ok(Box::new(self.iter_cast::<f64>().vclip(min, max)))
            }
            Median => {
                // default method is clip median - 3 * mad, median + 3 * mad
                let method_params = method_params.unwrap_or(3.);
                let median = self.vmedian();
                if median.not_none() {
                    let mad = self
                        .map(|v| (v.cast() - median).abs())
                        .collect_trusted_to_vec()
                        .vmedian();
                    let min = median - method_params * mad;
                    let max = median + method_params * mad;
                    Ok(Box::new(self.iter_cast::<f64>().vclip(min, max)))
                } else {
                    Ok(Box::new(self.iter_cast::<f64>()))
                }
            }
            Sigma => {
                // default method is clip mean - 3 * std, mean + 3 * std
                let method_params = method_params.unwrap_or(3.);
                let (mean, var) = self.to_iter().vmean_var(2);
                if mean.not_none() && var.not_none() && var > EPS {
                    let std = var.sqrt();
                    let min = mean - method_params * std;
                    let max = mean + method_params * std;
                    Ok(Box::new(self.iter_cast::<f64>().vclip(min, max)))
                } else {
                    Ok(Box::new(self.iter_cast::<f64>()))
                }
            }
        }
    }
}

impl<V: Vec1View<Item = T>, T: IsNone> MapValidFinal<T> for V {}

#[cfg(test)]
mod tests {
    #[cfg(feature = "agg")]
    use super::*;
    #[cfg(feature = "agg")]
    use tea_core::testing::assert_vec1d_equal_numeric;
    #[test]
    #[cfg(feature = "agg")]
    fn test_winsorize() -> TResult<()> {
        use super::*;
        let epsilon = Some(1e-10);
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b: Vec<_> = a.winsorize(WinsorizeMethod::Quantile, Some(0.1))?.collect();
        assert_vec1d_equal_numeric(&b, &vec![1.9, 2., 3., 4., 5., 6., 7., 8., 9., 9.1], epsilon);
        let b: Vec<_> = a.winsorize(WinsorizeMethod::Median, Some(1.))?.collect();
        assert_eq!(b, vec![3., 3., 3., 4., 5., 6., 7., 8., 8., 8.]);
        let b: Vec<_> = a.winsorize(WinsorizeMethod::Sigma, Some(1.))?.collect();
        assert_eq!(
            b,
            vec![
                2.4723496459025083,
                2.4723496459025083,
                3.,
                4.,
                5.,
                6.,
                7.,
                8.,
                8.527650354097492,
                8.527650354097492
            ]
        );
        Ok(())
    }
}
