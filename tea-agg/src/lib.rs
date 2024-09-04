mod vec_valid;

use tea_core::prelude::*;
pub use vec_valid::*;
/// Extension trait providing additional aggregation methods for iterables with potentially invalid (None) values.
pub trait AggValidExt<T: IsNone>: IntoIterator<Item = T> + Sized {
    /// Computes the sum of valid values filtered by a mask, along with the count of valid elements.
    ///
    /// # Arguments
    ///
    /// * `mask` - An iterable of boolean-like values used to filter the input.
    ///
    /// # Returns
    ///
    /// A tuple containing the count of valid elements and their sum.
    #[inline]
    fn n_vsum_filter<U, I>(self, mask: I) -> (usize, T::Inner)
    where
        I: IntoIterator<Item = U>,
        U: IsNone,
        U::Inner: Cast<bool>,
        T::Inner: Number,
    {
        self.into_iter()
            .zip(mask)
            .filter_map(|(v, flag)| {
                if flag.not_none() {
                    if flag.unwrap().cast() {
                        Some(v)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .vfold_n(T::Inner::zero(), |acc, x| acc + x)
    }

    /// Computes the sum of valid values filtered by a mask.
    ///
    /// # Arguments
    ///
    /// * `mask` - An iterable of boolean-like values used to filter the input.
    ///
    /// # Returns
    ///
    /// The sum of valid elements, or None if no valid elements are found.
    #[inline]
    fn n_sum_filter<U, I>(self, mask: I) -> Option<T::Inner>
    where
        I: IntoIterator<Item = U>,
        U: IsNone,
        U::Inner: Cast<bool>,
        T::Inner: Number,
    {
        let (n, sum) = self.n_vsum_filter(mask);
        if n > 0 {
            Some(sum)
        } else {
            None
        }
    }

    /// Computes the mean of valid values filtered by a mask.
    ///
    /// # Arguments
    ///
    /// * `mask` - An iterable of boolean-like values used to filter the input.
    /// * `min_periods` - The minimum number of valid elements required to compute the mean.
    ///
    /// # Returns
    ///
    /// The mean of valid elements, or NaN if the number of valid elements is less than `min_periods`.
    #[inline]
    fn vmean_filter<U, I>(self, mask: I, min_periods: usize) -> f64
    where
        I: IntoIterator<Item = U>,
        U: IsNone,
        U::Inner: Cast<bool>,
        T::Inner: Number,
    {
        let (n, sum) = self.n_vsum_filter(mask);
        if n >= min_periods {
            sum.f64() / n.f64()
        } else {
            f64::NAN
        }
    }

    /// Computes the kurtosis of the data.
    ///
    /// # Arguments
    ///
    /// * `min_periods` - The minimum number of valid elements required to compute the kurtosis.
    ///
    /// # Returns
    ///
    /// The kurtosis of the data, or NaN if the number of valid elements is less than `min_periods`.
    fn vkurt(self, min_periods: usize) -> f64
    where
        T::Inner: Number,
    {
        let (mut m1, mut m2, mut m3, mut m4) = (0., 0., 0., 0.);
        let n = self.vapply_n(|v| {
            let v = v.f64();
            m1 += v;
            let v2 = v * v;
            m2 += v2;
            m3 += v2 * v;
            m4 += v2 * v2;
        });
        if n < min_periods {
            return f64::NAN;
        }
        let mut res = if n >= 4 {
            let n_f64 = n.f64();
            m1 /= n_f64; // Ex
            m2 /= n_f64; // Ex^2
            let var = m2 - m1.powi(2);
            if var <= EPS {
                0.
            } else {
                let var2 = var.powi(2); // var^2
                m4 /= n_f64; // Ex^4
                m3 /= n_f64; // Ex^3
                let mean2_var = m1.powi(2) / var; // (mean / std)^2
                (m4 - 4. * m1 * m3) / var2 + 6. * mean2_var + 3. * mean2_var.powi(2)
            }
        } else {
            f64::NAN
        };
        if res.not_none() && res != 0. {
            res = 1. / ((n - 2) * (n - 3)).f64()
                * ((n.pow(2) - 1).f64() * res - (3 * (n - 1).pow(2)).f64())
        }
        res
    }
}

impl<I: IntoIterator<Item = T>, T: IsNone> AggValidExt<T> for I {}
