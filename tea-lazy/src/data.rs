use derive_more::From;
use std::sync::Arc;

use tevec::prelude::*;

#[derive(From, Clone)]
pub enum Data<'a> {
    TrustIter(Arc<DynTrustIter<'a>>),
    Scalar(Arc<Scalar>),
    Vec(Arc<DynVec>),
}

impl<'a> From<DynTrustIter<'a>> for Data<'a> {
    #[inline]
    fn from(iter: DynTrustIter<'a>) -> Self {
        Data::TrustIter(Arc::new(iter))
    }
}

impl<T: GetDataType> From<Vec<T>> for Data<'_> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        let vec: DynVec = vec.into();
        vec.into()
    }
}

impl From<DynVec> for Data<'_> {
    #[inline]
    fn from(vec: DynVec) -> Self {
        Data::Vec(Arc::new(vec))
    }
}

trait ScalarElement {}

impl ScalarElement for bool {}
impl ScalarElement for f32 {}
impl ScalarElement for f64 {}
impl ScalarElement for i32 {}
impl ScalarElement for i64 {}
impl ScalarElement for u8 {}
impl ScalarElement for u64 {}
impl ScalarElement for usize {}
impl ScalarElement for String {}
impl ScalarElement for Option<usize> {}
#[cfg(feature = "time")]
impl ScalarElement for DateTime {}
#[cfg(feature = "time")]
impl ScalarElement for TimeDelta {}

impl<T: ScalarElement> From<T> for Data<'_>
where
    T: Into<Scalar>,
{
    #[inline]
    fn from(v: T) -> Self {
        let s: Scalar = v.into();
        s.into()
    }
}

impl From<Scalar> for Data<'_> {
    #[inline]
    fn from(vec: Scalar) -> Self {
        Data::Scalar(Arc::new(vec))
    }
}

impl<'a> Data<'a> {
    pub fn try_into_iter(self) -> Result<DynTrustIter<'a>, Self> {
        match self {
            Data::TrustIter(iter) => Arc::try_unwrap(iter).map_err(|iter| iter.into()),
            Data::Vec(vec) => {
                match Arc::try_unwrap(vec) {
                    Ok(vec) => Ok(vec.into_iter().unwrap()),
                    Err(vec) => {
                        // the data is still shared
                        // this should only happen when the data is stored in a context
                        // so it is safe to reference data
                        let iter: DynTrustIter<'a> =
                            unsafe { std::mem::transmute(vec.to_iter().unwrap()) };
                        Ok(iter)
                    }
                }
            }
            Data::Scalar(scalar) => {
                match Arc::try_unwrap(scalar) {
                    Ok(scalar) => Ok(scalar.into_iter().unwrap()),
                    Err(scalar) => {
                        // the data is still shared
                        // this should only happen when the data is stored in a context
                        // so it is safe to reference data
                        let iter: DynTrustIter<'a> =
                            unsafe { std::mem::transmute(scalar.to_iter().unwrap()) };
                        Ok(iter)
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Context<'a> {
    pub data: Vec<Data<'a>>,
}

impl<'a> Context<'a> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
