use tea_core::prelude::*;
// use num_traits::Zero;

#[derive(Default)]
pub enum CorrMethod {
    #[default]
    Pearson,
    #[cfg(feature = "map")]
    Spearman,
}

pub trait Vec1AggValidExt<T: IsNone>: Vec1View<Item = T> + Sized {
    fn vcorr<V2: Vec1View<Item = T>>(
        &self,
        other: &V2,
        min_periods: Option<usize>,
        method: CorrMethod,
    ) -> T::Cast<f64>
    where
        T::Inner: Zero + Number,
        T: PartialEq,
    {
        let min_periods = min_periods.unwrap_or(self.len() / 2);
        match method {
            CorrMethod::Pearson => self.to_iter().vcorr_pearson(other.to_iter(), min_periods),
            #[cfg(feature = "map")]
            CorrMethod::Spearman => {
                use tea_map::MapValidVec;
                let v1_rank: Vec<f64> = self.vrank(false, false);
                let v2_rank: Vec<f64> = other.vrank(false, false);
                v1_rank.vcorr_pearson(v2_rank, min_periods).into_cast::<T>()
            }
        }
    }
}

impl<V: Vec1View<Item = T>, T: IsNone> Vec1AggValidExt<T> for V {}
