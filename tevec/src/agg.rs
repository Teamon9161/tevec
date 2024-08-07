pub use tea_agg::*;
use tea_core::prelude::*;
#[cfg(feature = "map")]
use tea_map::*;

#[derive(Default, Clone, Copy)]
pub enum CorrMethod {
    #[default]
    Pearson,
    #[cfg(feature = "map")]
    Spearman,
}

pub trait AggValidFinal<T: IsNone>: Vec1View<T> {
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

    #[cfg(feature = "map")]
    fn half_life(&self, min_periods: Option<usize>) -> usize
    where
        T: Clone,
        T::Inner: Number,
    {
        let mut n: usize;
        let mut last_n = 0;
        let mut i = 0;
        let min_periods = min_periods.unwrap_or(self.len() / 2);
        loop {
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
        let s: Vec<f64> = vec![];
        assert_eq!(s.ts_vmean::<Vec<f64>, _>(4, Some(1)).half_life(Some(1)), 1);
    }
}
