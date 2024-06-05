use std::sync::Arc;
use tevec::prelude::*;

pub enum Data {
    TrustIter(DynTrustIter),
}

impl From<DynTrustIter> for Data {
    #[inline]
    fn from(iter: DynTrustIter) -> Self {
        Data::TrustIter(iter)
    }
}

pub struct MapExpr {
    pub name: &'static str,
    pub func: Arc<dyn Fn(DynTrustIter) -> TResult<Data>>,
}

impl MapExpr {
    #[inline]
    pub fn eval(&self, input: Data) -> TResult<Data> {
        match input {
            Data::TrustIter(iter) => (self.func)(iter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn test_basic() -> TResult<()> {
        let e = MapExpr {
            name: "abs",
            func: Arc::new(|iter| Ok(iter.abs().into())),
        };
        let input: DynTrustIter = vec![-1.0, 2.0, -3.0].into_iter().into();
        let res = e.eval(input.into())?;
        match res {
            Data::TrustIter(iter) => {
                let data: Vec<f64> = iter.f64()?.collect_trusted_vec1();
                assert_eq!(data, vec![1.0, 2.0, 3.0]);
            }
        }
        Ok(())
    }
}
