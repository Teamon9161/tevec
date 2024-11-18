pub use tea_agg::*;
use tea_core::prelude::*;
#[cfg(feature = "map")]
use tea_map::*;
/// Enum representing different correlation methods.
#[derive(Default, Clone, Copy)]
pub enum CorrMethod {
    /// Pearson correlation coefficient (default).
    #[default]
    Pearson,
    /// Spearman's rank correlation coefficient.
    #[cfg(feature = "map")]
    Spearman,
}

/// Trait for aggregation operations on vectors with valid (non-None) elements.
pub trait AggValidFinal<T: IsNone>: Vec1View<T> {
    /// Calculates the correlation between two vectors using the specified method.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to correlate with.
    /// * `min_periods` - The minimum number of valid pairs required to compute the correlation.
    /// * `method` - The correlation method to use (Pearson or Spearman).
    ///
    /// # Returns
    ///
    /// The correlation coefficient as a floating-point number.
    #[cfg(feature = "map")]
    fn vcorr<V2: Vec1View<T>>(
        &self,
        other: &V2,
        min_periods: Option<usize>,
        method: CorrMethod,
    ) -> T::Cast<f64>
    where
        T::Inner: Zero + Number,
        T: PartialEq + PartialOrd,
        f64: Cast<T::Cast<f64>>,
        Self: MapValidVec<T>,
    {
        let min_periods = min_periods.unwrap_or(self.len() / 2);
        match method {
            CorrMethod::Pearson => self.titer().vcorr_pearson(other.titer(), min_periods),
            #[cfg(feature = "map")]
            CorrMethod::Spearman => {
                let v1_rank = self.vrank::<Vec<f64>, _>(false, false);
                let v2_rank = other.vrank::<Vec<f64>, _>(false, false);
                v1_rank.vcorr_pearson(v2_rank, min_periods)
            },
        }
    }

    /// Calculates the half-life of the autocorrelation of the vector.
    ///
    /// The half-life is defined as the lag at which the autocorrelation drops to 0.5.
    ///
    /// # Arguments
    ///
    /// * `min_periods` - The minimum number of valid pairs required to compute correlations.
    ///
    /// # Returns
    ///
    /// The half-life as a usize.
    #[cfg(feature = "map")]
    fn half_life(&self, min_periods: Option<usize>) -> usize
    where
        T: Clone,
        T::Inner: Number,
    {
        let mut n: usize = 0;
        let mut last_n = 0;
        let mut i = 0;
        let len = self.len();
        if len == 0 {
            return 0;
        }
        let min_periods = min_periods.unwrap_or(len / 2);
        while n < len {
            n = 2usize.pow(i);
            let s_shift = self.titer().vshift(n as i32, None);
            let corr: f64 = self.titer().vcorr_pearson(s_shift, min_periods);
            if (corr <= 0.5) || corr.is_nan() {
                break;
            } else {
                last_n = n;
            }
            i += 1;
        }
        n = n.min(self.len() - 1);
        let mut life: usize;
        while n - last_n > 1 {
            life = (n + last_n) / 2;
            let corr: f64 = self
                .titer()
                .vcorr_pearson(self.titer().vshift(life as i32, None), min_periods);
            if corr < 0.5 {
                (last_n, n) = (last_n, life);
            } else if corr > 0.5 {
                (last_n, n) = (life, last_n);
            } else {
                n = life;
                break;
            }
        }
        n
    }
}

impl<V: Vec1View<T>, T: IsNone> AggValidFinal<T> for V {}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "rolling", feature = "map"))]
    use crate::prelude::*;
    #[test]
    #[cfg(all(feature = "rolling", feature = "map"))]
    fn test_half_life() {
        let s = vec![10., 12., 13., 14., 15., 12., 11., 14., 15., 16.];
        assert_eq!(s.ts_vmean::<Vec<f64>, _>(4, Some(1)).half_life(Some(1)), 3);
        // test empty vec
        let s: Vec<f64> = vec![];
        assert_eq!(s.ts_vmean::<Vec<f64>, _>(4, Some(1)).half_life(Some(1)), 0);
        // test all nan
        let s = vec![f64::NAN; 10];
        assert_eq!(s.ts_vmean::<Vec<f64>, _>(4, Some(1)).half_life(Some(1)), 1);
    }
}
