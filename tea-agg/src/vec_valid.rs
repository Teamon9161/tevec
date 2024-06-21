use tea_core::prelude::*;

#[derive(Copy, Clone)]
pub enum QuantileMethod {
    Linear,
    Lower,
    Higher,
    MidPoint,
}

pub trait VecAggValidExt<T: IsNone>: Vec1View<T> {
    /// Calculate the quantile of the vector.
    /// return error if q is not between 0 and 1.
    fn vquantile(&self, q: f64, method: QuantileMethod) -> TResult<f64>
    where
        T: Cast<f64>,
        T::Inner: Number,
    {
        tensure!(
            (0. ..=1.).contains(&q),
            "q must be between 0 and 1, find {}",
            q
        );
        use QuantileMethod::*;
        let mut out_c: Vec<_> = self.titer().collect_trusted_vec1(); // clone the array
        let slc = out_c.try_as_slice_mut().unwrap();
        let n = AggValidBasic::count(self.titer());
        // fast path for special cases
        if n == 0 {
            return Ok(f64::NAN);
        } else if n == 1 {
            return Ok(slc[0].clone().cast());
        }
        let len_1 = (n - 1).f64();
        let (q, i, j, vi, vj) = if q <= 0.5 {
            let q_idx = len_1 * q;
            let (i, j) = (q_idx.floor().usize(), q_idx.ceil().usize());
            let (head, m, _tail) = slc.select_nth_unstable_by(j, |va, vb| va.sort_cmp(vb));
            if i != j {
                let vi: f64 = head.titer().vmax().map(|v| v.f64()).cast();
                (q, i, j, vi, m.clone().cast())
            } else {
                return Ok(m.clone().cast());
            }
        } else {
            // sort from largest to smallest
            let q = 1. - q;
            let q_idx = len_1 * q;
            let (i, j) = (q_idx.floor().usize(), q_idx.ceil().usize());
            let (head, m, _tail) = slc.select_nth_unstable_by(j, |va, vb| va.sort_cmp_rev(vb));
            if i != j {
                let vi: f64 = head.titer().vmin().map(|v| v.f64()).cast();
                match method {
                    Lower => {
                        return Ok(m.clone().cast());
                    }
                    Higher => {
                        return Ok(vi);
                    }
                    _ => {}
                };
                (q, i, j, vi, m.clone().cast())
            } else {
                return Ok(m.clone().cast());
            }
        };
        match method {
            Linear => {
                // `i + (j - i) * fraction`, where `fraction` is the
                // fractional part of the index surrounded by `i` and `j`.
                let (qi, qj) = (i.f64() / len_1, j.f64() / len_1);
                let fraction = (q - qi) / (qj - qi);
                Ok(vi + (vj - vi) * fraction)
            }
            Lower => Ok(vi),                // i
            Higher => Ok(vj),               // j
            MidPoint => Ok((vi + vj) / 2.), // (i + j) / 2.
        }
    }

    /// Calculate the median of the vector.
    #[inline]
    fn vmedian(&self) -> f64
    where
        T: Cast<f64>,
        T::Inner: Number,
    {
        self.vquantile(0.5, QuantileMethod::Linear).unwrap()
    }
}
impl<V: Vec1View<T>, T: IsNone> VecAggValidExt<T> for V {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_quantile() {
        use super::*;
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(a.vquantile(0.5, QuantileMethod::Linear).unwrap(), 5.5);
        assert_eq!(a.vquantile(0.5, QuantileMethod::Lower).unwrap(), 5.);
        assert_eq!(a.vquantile(0.5, QuantileMethod::Higher).unwrap(), 6.);
        assert_eq!(a.vquantile(0.5, QuantileMethod::MidPoint).unwrap(), 5.5);
        assert_eq!(a.vquantile(0.25, QuantileMethod::Linear).unwrap(), 3.25);
        assert_eq!(a.vquantile(0.25, QuantileMethod::Lower).unwrap(), 3.);
        assert_eq!(a.vquantile(0.25, QuantileMethod::Higher).unwrap(), 4.);
        assert_eq!(a.vquantile(0.25, QuantileMethod::MidPoint).unwrap(), 3.5);
        assert_eq!(a.vquantile(0.75, QuantileMethod::Linear).unwrap(), 7.75);
        assert_eq!(a.vquantile(0.75, QuantileMethod::Lower).unwrap(), 7.);
        assert_eq!(a.vquantile(0.75, QuantileMethod::Higher).unwrap(), 8.);
        assert_eq!(a.vquantile(0.75, QuantileMethod::MidPoint).unwrap(), 7.5);
        assert_eq!(a.vquantile(0.22, QuantileMethod::Linear).unwrap(), 2.98);
        assert_eq!(a.vquantile(0.78, QuantileMethod::Linear).unwrap(), 8.02);
    }
}
