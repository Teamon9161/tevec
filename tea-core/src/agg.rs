use num_traits::Zero;
use tea_dtype::{BoolType, Cast, IntoCast, IsNone, Number};

use crate::prelude::{EPS, IterBasic};

pub trait AggValidBasic<T: IsNone>: IntoIterator<Item = T> + Sized {
    #[inline]
    /// count the number of valid elements in the vector.
    #[deprecated(since = "0.3.0", note = "Please use count_valid instead")]
    fn count(self) -> usize {
        self.vfold_n((), |(), _| {}).0
    }

    #[inline]
    /// Counts the number of valid (non-None) elements in the iterator.
    ///
    /// This method iterates through all elements and counts those that are not None.
    ///
    /// # Returns
    ///
    /// Returns the count of valid elements as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), None, Some(2), None, Some(3)];
    /// assert_eq!(vec.count_valid(), 3);
    /// ```
    fn count_valid(self) -> usize {
        self.vfold_n((), |(), _| {}).0
    }

    #[inline]
    /// Finds the first valid (non-None) element in the iterator.
    ///
    /// This method iterates through the elements and returns the first one that is not None.
    ///
    /// # Returns
    ///
    /// Returns an `Option<T>`:
    /// - `Some(T)` if a valid element is found
    /// - `None` if no valid elements are found (i.e., all elements are None or the iterator is empty)
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![None, Some(1), None, Some(2), Some(3)];
    /// assert_eq!(vec.vfirst(), Some(Some(1)));
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vfirst(), None);
    /// ```
    fn vfirst(self) -> Option<T> {
        self.into_iter().find(|v| v.not_none())
    }

    #[inline]
    /// Finds the last valid (non-None) element in the iterator.
    ///
    /// This method iterates through the elements in reverse order and returns the first non-None element encountered.
    ///
    /// # Returns
    ///
    /// Returns an `Option<T>`:
    /// - `Some(T)` if a valid element is found
    /// - `None` if no valid elements are found (i.e., all elements are None or the iterator is empty)
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), None, Some(2), None, Some(3)];
    /// assert_eq!(vec.vlast(), Some(Some(3)));
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vlast(), None);
    /// ```
    ///
    /// # Note
    ///
    /// This method requires the iterator to be double-ended.
    fn vlast(self) -> Option<T>
    where
        Self::IntoIter: DoubleEndedIterator,
    {
        self.into_iter().rev().find(|v| v.not_none())
    }

    #[inline]
    /// Counts the number of `None` values in the iterator.
    ///
    /// This method iterates through all elements and counts those that are `None`.
    ///
    /// # Returns
    ///
    /// Returns the count of `None` values as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), None, Some(2), None, Some(3)];
    /// assert_eq!(vec.count_none(), 2);
    /// ```
    fn count_none(self) -> usize {
        let mut n = 0;
        self.into_iter().for_each(|v| {
            if v.is_none() {
                n += 1;
            }
        });
        n
    }

    #[inline]
    /// Counts the number of occurrences of a specific value in the iterator.
    ///
    /// This method iterates through all elements and counts those that match the given value.
    /// It handles both `Some` and `None` values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to count occurrences of. It can be either `Some(T::Inner)` or `None`.
    ///
    /// # Returns
    ///
    /// Returns the count of occurrences as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), None, Some(2), Some(1), None, Some(3)];
    /// assert_eq!(vec.titer().vcount_value(Some(1)), 2);
    /// assert_eq!(vec.titer().vcount_value(None), 2);
    /// assert_eq!(vec.vcount_value(Some(4)), 0);
    /// ```
    fn vcount_value(self, value: T) -> usize
    where
        T::Inner: PartialEq,
    {
        if value.not_none() {
            let value = value.unwrap();
            self.vfold(0, |acc, x| if x.unwrap() == value { acc + 1 } else { acc })
        } else {
            self.into_iter()
                .fold(0, |acc, x| if x.is_none() { acc + 1 } else { acc })
        }
    }

    #[inline]
    /// Checks if any valid (non-None) element in the iterator satisfies the given condition.
    ///
    /// This method iterates through all elements and returns `true` if any element is not None and satisfies the condition.
    ///
    /// # Returns
    ///
    /// Returns a `bool`:
    /// - `true` if any valid element satisfies the condition
    /// - `false` if no valid elements satisfy the condition or the iterator is empty
    fn vany(self) -> bool
    where
        T::Inner: BoolType,
    {
        self.vfold(false, |acc, x| acc || x.unwrap().bool_())
    }

    #[inline]
    /// Checks if all valid (non-None) elements in the iterator satisfy the given condition.
    ///
    /// This method iterates through all elements and returns `true` if all elements are not None and satisfy the condition.
    ///
    /// # Returns
    ///
    /// Returns a `bool`:
    /// - `true` if all valid elements satisfy the condition
    /// - `false` if any valid element does not satisfy the condition or the iterator is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(true), Some(true), Some(false)];
    /// assert_eq!(vec.vall(), false);
    ///
    /// let vec = vec![Some(true), Some(true), Some(true)];
    /// assert_eq!(vec.vall(), true);
    ///
    /// let empty_vec: Vec<Option<bool>> = vec![];
    /// assert_eq!(empty_vec.vall(), true); // All elements are None, so it satisfies the condition
    /// ```
    fn vall(self) -> bool
    where
        T::Inner: BoolType,
    {
        self.vfold(true, |acc, x| acc && x.unwrap().bool_())
    }

    #[inline]
    /// Returns the sum of all valid elements in the vector.
    ///
    /// This method iterates through all elements, summing up the valid (non-None) values.
    ///
    /// # Returns
    ///
    /// Returns an `Option<T::Inner>`:
    /// - `Some(sum)` if there is at least one valid element
    /// - `None` if there are no valid elements or the vector is empty
    ///
    /// # Type Parameters
    ///
    /// - `T::Inner`: Must implement the `Zero` trait to provide a zero value for initialization
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(2), None, Some(3)];
    /// assert_eq!(vec.vsum(), Some(6));
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vsum(), None);
    ///
    /// let all_none_vec: Vec<Option<f64>> = vec![None, None, None];
    /// assert_eq!(all_none_vec.vsum(), None);
    /// ```
    fn vsum(self) -> Option<T::Inner>
    where
        T::Inner: Zero,
    {
        let (n, sum) = self.vfold_n(T::Inner::zero(), |acc, x| acc + x);
        if n >= 1 { Some(sum) } else { None }
    }

    #[inline]
    /// Calculates the mean (average) of all valid elements in the vector.
    ///
    /// This method iterates through all elements, summing up the valid (non-None) values
    /// and counting the number of valid elements. It then calculates the mean by dividing
    /// the sum by the count.
    ///
    /// # Returns
    ///
    /// Returns an `f64`:
    /// - The calculated mean if there is at least one valid element
    /// - `f64::NAN` if there are no valid elements or the vector is empty
    ///
    /// # Type Parameters
    ///
    /// - `T::Inner`: Must implement the `Number` trait to support arithmetic operations
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(2), None, Some(3)];
    /// assert_eq!(vec.vmean(), 2.0);
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert!(empty_vec.vmean().is_nan());
    ///
    /// let all_none_vec: Vec<Option<f64>> = vec![None, None, None];
    /// assert!(all_none_vec.vmean().is_nan());
    /// ```
    fn vmean(self) -> f64
    where
        T::Inner: Number,
    {
        let (n, sum) = self.vfold_n(T::Inner::zero(), |acc, x| acc + x);
        if n >= 1 {
            sum.f64() / n as f64
        } else {
            f64::NAN
        }
    }

    /// Calculates the mean and variance of all valid elements in the vector.
    ///
    /// This method iterates through all elements, computing the sum and sum of squares
    /// of valid (non-None) values. It then calculates the mean and variance using these values.
    ///
    /// # Arguments
    ///
    /// * `min_periods` - The minimum number of valid observations required to calculate the result.
    ///
    /// # Returns
    ///
    /// Returns a tuple of two `f64` values:
    /// - The first element is the calculated mean
    /// - The second element is the calculated variance
    /// - Both elements are `f64::NAN` if there are fewer valid elements than `min_periods`
    ///
    /// # Type Parameters
    ///
    /// - `T::Inner`: Must implement the `Number` trait to support arithmetic operations
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(2), None, Some(3)];
    /// let (mean, var) = vec.vmean_var(1);
    /// assert_eq!(mean, 2.0);
    /// assert!((var - 1.0).abs() < EPS);
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// let (mean, var) = empty_vec.vmean_var(1);
    /// assert!(mean.is_nan() && var.is_nan());
    /// ```
    fn vmean_var(self, min_periods: usize) -> (f64, f64)
    where
        T::Inner: Number,
    {
        let (mut m1, mut m2) = (0., 0.);
        let n = self.vapply_n(|v| {
            let v = v.f64();
            m1 += v;
            m2 += v * v;
        });
        if n < min_periods {
            return (f64::NAN, f64::NAN);
        }
        let n_f64 = n.f64();
        m1 /= n_f64; // E(x)
        m2 /= n_f64; // E(x^2)
        m2 -= m1.powi(2); // variance = E(x^2) - (E(x))^2
        if m2 <= EPS {
            (m1, 0.)
        } else if n >= 2 {
            (m1, m2 * n_f64 / (n - 1).f64())
        } else {
            (f64::NAN, f64::NAN)
        }
    }

    /// Calculates the variance of the data.
    ///
    /// This method computes the variance of the non-null values in the collection.
    ///
    /// # Arguments
    ///
    /// * `min_periods` - The minimum number of non-null values required to calculate the variance.
    ///   If the number of non-null values is less than `min_periods`, the method returns `f64::NAN`.
    ///
    /// # Returns
    ///
    /// Returns an `f64` representing the variance of the data. If there are fewer valid elements
    /// than `min_periods`, returns `f64::NAN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(2), None, Some(3)];
    /// let var = vec.vvar(1);
    /// assert!((var - 1.0).abs() < EPS);
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// let var = empty_vec.vvar(1);
    /// assert!(var.is_nan());
    /// ```
    #[inline]
    fn vvar(self, min_periods: usize) -> f64
    where
        T::Inner: Number,
    {
        self.vmean_var(min_periods).1
    }

    /// Calculates the standard deviation of the data.
    ///
    /// This method computes the standard deviation of the non-null values in the collection.
    ///
    /// # Arguments
    ///
    /// * `min_periods` - The minimum number of non-null values required to calculate the standard deviation.
    ///   If the number of non-null values is less than `min_periods`, the method returns `f64::NAN`.
    ///
    /// # Returns
    ///
    /// Returns an `f64` representing the standard deviation of the data. If there are fewer valid elements
    /// than `min_periods`, returns `f64::NAN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(2), None, Some(3)];
    /// let std = vec.vstd(1);
    /// assert!((std - 1.0).abs() < EPS);
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// let std = empty_vec.vstd(1);
    /// assert!(std.is_nan());
    /// ```
    #[inline]
    fn vstd(self, min_periods: usize) -> f64
    where
        T::Inner: Number,
    {
        self.vvar(min_periods).sqrt()
    }

    /// Calculates the skewness of the data.
    ///
    /// Skewness is a measure of the asymmetry of the probability distribution of a real-valued random variable about its mean.
    ///
    /// # Arguments
    ///
    /// * `min_periods` - The minimum number of non-null values required to calculate the skewness.
    ///   If the number of non-null values is less than `min_periods`, the method returns `f64::NAN`.
    ///
    /// # Returns
    ///
    /// Returns an `f64` representing the skewness of the data. If there are fewer valid elements
    /// than `min_periods`, or if the number of elements is less than 3, returns `f64::NAN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    /// let skew = vec.vskew(1);
    /// assert!((skew - 0.0).abs() < EPS);
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// let skew = empty_vec.vskew(1);
    /// assert!(skew.is_nan());
    /// ```
    fn vskew(self, min_periods: usize) -> f64
    where
        T::Inner: Number,
    {
        let (mut m1, mut m2, mut m3) = (0., 0., 0.);
        let n = self.vapply_n(|v| {
            let v = v.f64();
            m1 += v;
            let v2 = v * v;
            m2 += v2;
            m3 += v2 * v;
        });
        if n < min_periods {
            return f64::NAN;
        }
        let mut res = if n >= 3 {
            let n_f64 = n.f64();
            m1 /= n_f64; // Ex
            m2 /= n_f64; // Ex^2
            let var = m2 - m1.powi(2);
            if var <= EPS {
                0.
            } else {
                let std = var.sqrt(); // var^2
                m3 /= n_f64; // Ex^3
                let mean_std = m1 / std; // mean / std
                m3 / std.powi(3) - 3_f64 * mean_std - mean_std.powi(3)
            }
        } else {
            f64::NAN
        };
        if res.not_none() && res != 0. {
            let adjust = (n * (n - 1)).f64().sqrt() / (n - 2).f64();
            res *= adjust;
        }
        res
    }

    /// Returns the maximum value in the vector.
    ///
    /// This method iterates through the vector and returns the maximum value.
    /// If the vector is empty or contains only `None` values, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(5), Some(3), Some(2), Some(4)];
    /// assert_eq!(vec.vmax(), Some(5));
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vmax(), None);
    ///
    /// let none_vec: Vec<Option<i32>> = vec![None, None, None];
    /// assert_eq!(none_vec.vmax(), None);
    /// ```
    #[inline]
    fn vmax(self) -> Option<T::Inner>
    where
        T::Inner: Number,
    {
        self.vfold(None, |acc, x| match acc.to_opt() {
            None => Some(x.unwrap()),
            Some(v) => Some(v.max_with(x.unwrap())),
        })
    }

    /// Returns the minimum value in the vector.
    ///
    /// This method iterates through the vector and returns the minimum value.
    /// If the vector is empty or contains only `None` values, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(5), Some(3), Some(2), Some(4)];
    /// assert_eq!(vec.vmin(), Some(1));
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vmin(), None);
    ///
    /// let none_vec: Vec<Option<i32>> = vec![None, None, None];
    /// assert_eq!(none_vec.vmin(), None);
    /// ```
    #[inline]
    fn vmin(self) -> Option<T::Inner>
    where
        T::Inner: Number,
    {
        self.vfold(None, |acc, x| match acc {
            None => Some(x.unwrap()),
            Some(v) => Some(v.min_with(x.unwrap())),
        })
    }

    /// Returns the index of the maximum value in the vector.
    ///
    /// This method iterates through the vector and returns the index of the maximum value.
    /// If the vector is empty or contains only `None` values, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(1), Some(5), Some(3), Some(2), Some(4)];
    /// assert_eq!(vec.vargmax(), Some(1)); // Index of 5
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vargmax(), None);
    ///
    /// let none_vec: Vec<Option<i32>> = vec![None, None, None];
    /// assert_eq!(none_vec.vargmax(), None);
    /// ```
    #[inline]
    fn vargmax(self) -> Option<usize>
    where
        T::Inner: PartialOrd,
    {
        use std::cmp::Ordering;
        let mut max = None;
        let mut max_idx = None;
        let mut current_idx = 0;
        self.into_iter().for_each(|v| {
            if v.not_none() {
                let v = v.unwrap();
                if let Some(max_value) = &max {
                    // None is smaller than any value
                    if let Some(Ordering::Greater) = v.partial_cmp(max_value) {
                        max = Some(v);
                        max_idx = Some(current_idx);
                    }
                } else {
                    max = Some(v);
                    max_idx = Some(current_idx);
                }
            }
            current_idx += 1;
        });
        max_idx
    }

    /// Returns the index of the minimum value in the vector.
    ///
    /// This method iterates through the vector and returns the index of the minimum value.
    /// If the vector is empty or contains only `None` values, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let vec = vec![Some(3), Some(1), Some(4), Some(2), Some(5)];
    /// assert_eq!(vec.vargmin(), Some(1)); // Index of 1
    ///
    /// let empty_vec: Vec<Option<i32>> = vec![];
    /// assert_eq!(empty_vec.vargmin(), None);
    ///
    /// let none_vec: Vec<Option<i32>> = vec![None, None, None];
    /// assert_eq!(none_vec.vargmin(), None);
    /// ```
    #[inline]
    fn vargmin(self) -> Option<usize>
    where
        T::Inner: PartialOrd,
    {
        use std::cmp::Ordering;
        let mut min = None;
        let mut min_idx = None;
        let mut current_idx = 0;
        self.into_iter().for_each(|v| {
            if v.not_none() {
                let v = v.unwrap();
                if let Some(min_value) = &min {
                    if let Some(Ordering::Less) = v.partial_cmp(min_value) {
                        min = Some(v);
                        min_idx = Some(current_idx);
                    }
                } else {
                    min = Some(v);
                    min_idx = Some(current_idx);
                }
            }
            current_idx += 1;
        });
        min_idx
    }

    /// Calculates the covariance between two vectors.
    ///
    /// This method computes the covariance between the elements of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An iterator over the second vector of values.
    /// * `min_periods` - The minimum number of pairs of non-None values required to have a valid result.
    ///
    /// # Returns
    ///
    /// Returns the covariance as `T::Cast<f64>`. If there are fewer valid pairs than `min_periods`,
    /// or if the computation results in NaN, returns `None`.
    ///
    /// # Type Parameters
    ///
    /// * `V2` - The type of the iterator for the second vector.
    /// * `T2` - The type of elements in the second vector, which must implement `IsNone`.
    fn vcov<V2: IntoIterator<Item = T2>, T2: IsNone>(
        self,
        other: V2,
        min_periods: usize,
    ) -> T::Cast<f64>
    where
        T::Inner: Number,
        T2::Inner: Number,
    {
        let (mut sum_a, mut sum_b, mut sum_ab) = (0., 0., 0.);
        let mut n = 0;
        let min_periods = min_periods.max_with(2);
        self.into_iter().zip(other).for_each(|(va, vb)| {
            if va.not_none() && vb.not_none() {
                n += 1;
                let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                sum_a += va;
                sum_b += vb;
                sum_ab += va * vb;
            }
        });
        if n >= min_periods {
            let res = (sum_ab - (sum_a * sum_b) / n as f64) / (n - 1) as f64;
            res.into_cast::<T>()
        } else {
            T::Cast::<f64>::none()
        }
    }

    /// Calculates the Pearson correlation coefficient between two vectors.
    ///
    /// This method computes the Pearson correlation coefficient between the elements of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An iterator over the second vector of values.
    /// * `min_periods` - The minimum number of pairs of non-None values required to have a valid result.
    ///
    /// # Returns
    ///
    /// Returns the Pearson correlation coefficient as type `O`. If there are fewer valid pairs than `min_periods`,
    /// or if the computation results in NaN (e.g., due to zero variance), returns `NaN`.
    ///
    /// # Type Parameters
    ///
    /// * `O` - The output type for the correlation coefficient.
    /// * `V2` - The type of the iterator for the second vector.
    /// * `T2` - The type of elements in the second vector, which must implement `IsNone`.
    fn vcorr_pearson<O, V2: IntoIterator<Item = T2>, T2: IsNone>(
        self,
        other: V2,
        min_periods: usize,
    ) -> O
    where
        T::Inner: Number,
        T2::Inner: Number,
        f64: Cast<O>,
    {
        let (mut sum_a, mut sum2_a, mut sum_b, mut sum2_b, mut sum_ab) = (0., 0., 0., 0., 0.);
        let mut n = 0;
        let min_periods = min_periods.max_with(2);
        self.into_iter().zip(other).for_each(|(va, vb)| {
            if va.not_none() && vb.not_none() {
                n += 1;
                let (va, vb) = (va.unwrap().f64(), vb.unwrap().f64());
                sum_a += va;
                sum2_a += va * va;
                sum_b += vb;
                sum2_b += vb * vb;
                sum_ab += va * vb;
            }
        });
        if n >= min_periods {
            let n = n.f64();
            let mean_a = sum_a / n;
            let mut var_a = sum2_a / n;
            let mean_b = sum_b / n;
            let mut var_b = sum2_b / n;
            var_a -= mean_a.powi(2);
            var_b -= mean_b.powi(2);
            if (var_a > EPS) & (var_b > EPS) {
                let exy = sum_ab / n;
                let exey = sum_a * sum_b / (n * n);
                let res = (exy - exey) / (var_a * var_b).sqrt();
                res.cast()
            } else {
                f64::NAN.cast()
            }
        } else {
            f64::NAN.cast()
        }
    }
}

pub trait AggBasic: IntoIterator + Sized {
    /// Counts the occurrences of a specific value in the iterator.
    ///
    /// This method iterates over the elements of the collection and counts
    /// how many times the specified `value` appears.
    ///
    /// # Arguments
    ///
    /// * `self` - The iterator to count values from.
    /// * `value` - The value to count occurrences of.
    ///
    /// # Returns
    ///
    /// Returns the number of times the specified `value` appears in the iterator.
    ///
    /// # Type Parameters
    ///
    /// * `Self::Item` - The type of items in the iterator, which must implement `PartialEq`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let numbers = vec![1, 2, 3, 2, 4, 2];
    /// assert_eq!(numbers.titer().count_value(2), 3);
    /// assert_eq!(numbers.count_value(5), 0);
    /// ```
    #[inline]
    fn count_value(self, value: Self::Item) -> usize
    where
        Self::Item: PartialEq,
    {
        self.into_iter()
            .fold(0, |acc, x| if x == value { acc + 1 } else { acc })
    }

    /// Checks if any element in the iterator satisfies a condition.
    ///
    /// This method returns `true` if at least one element in the iterator
    /// evaluates to `true` when converted to a boolean value.
    ///
    /// # Returns
    ///
    /// Returns `true` if any element is `true`, `false` otherwise.
    ///
    /// # Type Parameters
    ///
    /// * `Self::Item` - The type of items in the iterator, which must implement `BoolType`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let values = vec![false, false, true, false];
    /// assert_eq!(values.any(), true);
    ///
    /// let empty: Vec<bool> = vec![];
    /// assert_eq!(empty.any(), false);
    /// ```
    #[inline]
    fn any(self) -> bool
    where
        Self::Item: BoolType,
    {
        Iterator::any(&mut self.into_iter(), |x| x.bool_())
    }

    /// Checks if all elements in the iterator satisfy a condition.
    ///
    /// This method returns `true` if all elements in the iterator
    /// evaluate to `true` when converted to a boolean value.
    ///
    /// # Returns
    ///
    /// Returns `true` if all elements are `true`, `false` otherwise.
    ///
    /// # Type Parameters
    ///
    /// * `Self::Item` - The type of items in the iterator, which must implement `BoolType`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let values = vec![true, true, true];
    /// assert_eq!(values.all(), true);
    ///
    /// let mixed = vec![true, false, true];
    /// assert_eq!(mixed.all(), false);
    ///
    /// let empty: Vec<bool> = vec![];
    /// assert_eq!(empty.all(), true);
    /// ```
    #[inline]
    fn all(self) -> bool
    where
        Self::Item: BoolType,
    {
        Iterator::all(&mut self.into_iter(), |x| x.bool_())
    }

    #[inline]
    /// Returns the first element of the iterator.
    /// If the iterator is empty, returns None.
    fn first(self) -> Option<Self::Item> {
        self.into_iter().next()
    }

    #[inline]
    /// Returns the last element of the iterator.
    /// If the iterator is empty, returns None.
    fn last(self) -> Option<Self::Item>
    where
        Self::IntoIter: DoubleEndedIterator,
    {
        self.into_iter().rev().first()
    }

    #[inline]
    /// Returns the sum of all elements in the vector.
    fn n_sum(self) -> (usize, Option<Self::Item>)
    where
        Self::Item: Zero,
    {
        let mut n = 0;
        let sum = self.into_iter().fold(Self::Item::zero(), |acc, x| {
            n += 1;
            acc + x
        });
        if n >= 1 { (n, Some(sum)) } else { (n, None) }
    }

    #[inline]
    /// Returns the sum of all elements in the vector.
    fn sum(self) -> Option<Self::Item>
    where
        Self::Item: Zero,
    {
        self.n_sum().1
    }

    /// Returns the mean of all elements in the iterator.
    ///
    /// This method calculates the arithmetic mean of all elements in the iterator.
    /// It first computes the sum and count of all elements using the `n_sum` method,
    /// then divides the sum by the count to get the mean.
    ///
    /// # Type Parameters
    ///
    /// - `Self::Item`: Must implement `Zero` and be castable to `f64`.
    ///
    /// # Returns
    ///
    /// - `Some(f64)`: The mean value if the iterator is not empty.
    /// - `None`: If the iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// assert_eq!(numbers.titer().mean(), Some(3.0));
    ///
    /// let empty: Vec<f64> = vec![];
    /// assert_eq!(empty.titer().mean(), None);
    /// ```
    #[inline]
    fn mean(self) -> Option<f64>
    where
        Self::Item: Zero + Cast<f64>,
    {
        let (len, sum) = self.n_sum();
        sum.map(|v| v.cast() / len as f64)
    }

    /// Returns the maximum element in the iterator.
    ///
    /// This method iterates through all elements and returns the maximum value.
    ///
    /// # Type Parameters
    ///
    /// - `Self::Item`: Must implement the `Number` trait.
    ///
    /// # Returns
    ///
    /// - `Some(Self::Item)`: The maximum value if the iterator is not empty.
    /// - `None`: If the iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use std::iter::empty;
    ///
    /// let numbers = vec![1, 5, 3, 2, 4];
    /// assert_eq!(AggBasic::max(numbers.titer()), Some(5));
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert_eq!(AggBasic::max(empty.titer()), None);
    /// ```
    #[inline]
    fn max(self) -> Option<Self::Item>
    where
        Self::Item: Number,
    {
        self.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(v) => Some(v.max_with(x)),
        })
    }

    /// Returns the minimum element in the iterator.
    ///
    /// This method iterates through all elements and returns the minimum value.
    ///
    /// # Type Parameters
    ///
    /// - `Self::Item`: Must implement the `Number` trait.
    ///
    /// # Returns
    ///
    /// - `Some(Self::Item)`: The minimum value if the iterator is not empty.
    /// - `None`: If the iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    /// use std::iter::empty;
    /// let numbers = vec![5, 1, 3, 2, 4];
    /// assert_eq!(AggBasic::min(numbers.titer()), Some(1));
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert_eq!(AggBasic::min(empty.titer()), None);
    /// ```
    #[inline]
    fn min(self) -> Option<Self::Item>
    where
        Self::Item: Number,
    {
        self.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(v) => Some(v.min_with(x)),
        })
    }

    /// Returns the index of the maximum element in the iterator.
    ///
    /// This method iterates through all elements and returns the index of the maximum value.
    ///
    /// # Type Parameters
    ///
    /// - `Self::Item`: Must implement `PartialOrd`.
    ///
    /// # Returns
    ///
    /// - `Some(usize)`: The index of the maximum value if the iterator is not empty.
    /// - `None`: If the iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let numbers = vec![1, 5, 3, 2, 4];
    /// assert_eq!(numbers.titer().argmax(), Some(1));
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert_eq!(empty.titer().argmax(), None);
    /// ```
    #[inline]
    fn argmax(self) -> Option<usize>
    where
        Self::Item: PartialOrd,
    {
        use std::cmp::Ordering;
        let mut max = None;
        let mut max_idx = None;
        let mut current_idx = 0;
        self.into_iter().for_each(|v| {
            if let Some(max_value) = &max {
                if let Some(Ordering::Greater) = v.partial_cmp(max_value) {
                    max = Some(v);
                    max_idx = Some(current_idx);
                }
            } else {
                max = Some(v);
                max_idx = Some(current_idx);
            }
            current_idx += 1;
        });
        max_idx
    }

    /// Returns the index of the minimum element in the iterator.
    ///
    /// This method iterates through all elements and returns the index of the minimum value.
    ///
    /// # Type Parameters
    ///
    /// - `Self::Item`: Must implement `PartialOrd`.
    ///
    /// # Returns
    ///
    /// - `Some(usize)`: The index of the minimum value if the iterator is not empty.
    /// - `None`: If the iterator is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_core::prelude::*;
    ///
    /// let numbers = vec![5, 1, 3, 2, 4];
    /// assert_eq!(numbers.titer().argmin(), Some(1));
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert_eq!(empty.titer().argmin(), None);
    /// ```
    #[inline]
    fn argmin(self) -> Option<usize>
    where
        Self::Item: PartialOrd,
    {
        use std::cmp::Ordering;
        let mut min = None;
        let mut min_idx = None;
        let mut current_idx = 0;
        self.into_iter().for_each(|v| {
            if let Some(min_value) = &min {
                if let Some(Ordering::Less) = v.partial_cmp(min_value) {
                    min = Some(v);
                    min_idx = Some(current_idx);
                }
            } else {
                min = Some(v);
                min_idx = Some(current_idx);
            }
            current_idx += 1;
        });
        min_idx
    }
}

impl<I: IntoIterator> AggBasic for I {}
impl<I: IntoIterator<Item = T>, T: IsNone> AggValidBasic<T> for I {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_sum() {
        let data: Vec<i32> = vec![];
        assert_eq!(AggBasic::sum(data.titer()), None);
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(AggBasic::sum(data.titer()), Some(15));
        assert_eq!(data.titer().mean(), Some(3.));
        assert_eq!(data.titer().vsum(), Some(15));
        assert_eq!(data.opt().vmean(), 3.);
        let data = vec![1., f64::NAN, 3.];
        assert!(AggBasic::sum(data.titer()).unwrap().is_nan());
        assert_eq!(data.titer().vsum(), Some(4.));
        assert_eq!(AggValidBasic::vsum(&data.opt()), Some(4.));
        assert_eq!(data.opt().vmean(), 2.);
        // #[cfg(feature = "ndarray")]
        // {
        //     use ndarray::prelude::*;
        //     let arr = arr0(1.);
        //     assert_eq!(arr.titer().vsum(), Some(1.));
        //     assert_eq!(arr.titer().vmean(), Some(1.))
        // }
    }

    #[test]
    fn test_cmp() {
        let data = vec![1., 3., f64::NAN, 2., 5.];
        assert_eq!(AggBasic::max(data.titer()), Some(5.));
        assert_eq!(AggBasic::min(data.titer()), Some(1.));
        assert_eq!(data.opt().vmax(), Some(5.));
        assert_eq!(data.opt().vmin(), Some(1.));
        let data: Vec<_> = data.opt().collect_trusted_vec1();
        assert_eq!(data.titer().vmax(), Some(5.));
        assert_eq!(data.vmin(), Some(1.));
        let data = vec![Some(1.), Some(2.), None, Some(3.)];
        assert_eq!(data.titer().vmin(), Some(1.));
    }

    #[test]
    fn test_count() {
        let data = vec![1., 2., f64::NAN, 2., f64::NAN, f64::NAN];
        assert_eq!(data.opt().count_valid(), 3);
        assert_eq!(data.opt().count_none(), 3);
        assert_eq!(data.titer().count_value(1.), 1);
        assert_eq!(data.titer().count_value(2.), 2);
        assert_eq!(data.titer().vcount_value(1.), 1);
        assert_eq!(data.titer().vcount_value(f64::NAN), 3);
        assert_eq!((data.opt().vcount_value(Some(2.))), 2);
        assert_eq!((data.opt().vcount_value(None)), 3);
    }

    #[test]
    fn test_bool() {
        let data = vec![true, false, false, false];
        assert!(data.titer().any());
        assert!(data.titer().vany());
        assert!(data.opt().vany());
        let data = vec![false, false, false, false];
        assert!(!data.titer().any());
        assert!(!data.titer().vany());
        assert!(!data.opt().vany());
        let data = vec![true, true, true, true];
        assert!(data.titer().all());
        assert!(data.titer().vall());
        assert!(data.opt().vall());
        let data = vec![true, false, true, true];
        assert!(!data.titer().all());
        assert!(!data.titer().vall());
        assert!(!data.opt().vall());
    }

    #[test]
    fn test_argmax_and_argmin() {
        let data = vec![2, 1, 3, -3];
        assert_eq!(data.titer().vargmax(), Some(2));
        assert_eq!(data.titer().vargmin(), Some(3));
        assert_eq!(data.titer().argmax(), Some(2));
        assert_eq!(data.titer().argmin(), Some(3));
        let data = vec![Some(5.), Some(2.), None, Some(1.), Some(-3.), Some(4.)];
        assert_eq!(data.titer().vargmax(), Some(0));
        assert_eq!(data.titer().vargmin(), Some(4));
    }
}
