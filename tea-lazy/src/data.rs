use derive_more::From;
use std::sync::Arc;

use tevec::{match_trust_iter, prelude::*};

#[derive(From, Clone)]
pub enum Data<'a> {
    TrustIter(Arc<DynTrustIter<'a>>),
    Scalar(Arc<Scalar>),
    Vec(Arc<DynVec>),
    Array(Arc<DynArray<'a>>),
}

impl<'a> From<DynTrustIter<'a>> for Data<'a> {
    #[inline]
    fn from(iter: DynTrustIter<'a>) -> Self {
        Data::TrustIter(Arc::new(iter))
    }
}

impl<'a> From<DynArray<'a>> for Data<'a> {
    #[inline]
    fn from(arr: DynArray<'a>) -> Self {
        Data::Array(Arc::new(arr))
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
    #[inline]
    pub fn into_scalar(self) -> TResult<Scalar> {
        self.try_into_scalar()
            .map_err(|_| terr!("Data cannot be converted into scalar"))
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> TResult<DynTrustIter<'a>> {
        self.try_into_iter()
            .map_err(|_| terr!("Data cannot be converted into iterator"))
    }

    #[inline]
    #[allow(clippy::missing_transmute_annotations)]
    pub fn into_array(self) -> TResult<DynArray<'a>> {
        if let Data::Array(array) = self {
            match Arc::try_unwrap(array) {
                Ok(array) => Ok(array),
                Err(array) => {
                    // the data is still shared
                    // this should only happen when the data is stored in a context
                    // so it is safe to reference data
                    Ok(unsafe { std::mem::transmute(array.view()) })
                }
            }
        } else {
            // TODO: Maybe we can convert iterator and scalar to vec
            // and then directly convert to array?
            tbail!("Data is not an array")
        }
    }

    #[inline]
    pub fn into_vec(self) -> TResult<DynVec> {
        if let Data::Vec(vec) = self {
            match Arc::try_unwrap(vec) {
                Ok(vec) => Ok(vec),
                Err(_) => {
                    tbail!("Can not convert data into vector as it is shared")
                }
            }
        } else {
            // TODO: Iterator and Scalar can be converted to Vec
            tbail!("Data is not an vector")
        }
    }

    pub fn try_into_scalar(self) -> Result<Scalar, Self> {
        match self {
            Data::TrustIter(iter) => {
                let iter = Arc::try_unwrap(iter).map_err(Data::TrustIter)?;
                let out: Scalar = match_trust_iter!(iter; dynamic(i) => {
                    let vec = i.collect_trusted_to_vec();
                    if vec.len() == 1 {
                        Ok(vec.into_iter().next().unwrap().into())
                    } else {
                        return Err(vec.into())
                    }
                },)
                .unwrap();
                Ok(out)
            }
            Data::Scalar(scalar) => match Arc::try_unwrap(scalar) {
                Ok(s) => Ok(s),
                Err(s) => s.cheap_clone().ok_or_else(|| Data::Scalar(s)),
            },
            Data::Vec(vec) => {
                if vec.len() == 1 {
                    Ok(vec.get(0).unwrap())
                } else {
                    Err(vec.into())
                }
            }
            // #[cfg(feature = "ndarray")]
            Data::Array(array) => {
                if array.len() == 1 {
                    Ok(array.get(0).unwrap())
                } else {
                    Err(array.into())
                }
            }
        }
    }

    pub fn try_into_iter(self) -> Result<DynTrustIter<'a>, Self> {
        match self {
            Data::TrustIter(iter) => Arc::try_unwrap(iter).map_err(|iter| iter.into()),
            Data::Vec(vec) => {
                match Arc::try_unwrap(vec) {
                    Ok(vec) => Ok(vec.into_titer().unwrap()),
                    Err(vec) => {
                        // the data is still shared
                        // this should only happen when the data is stored in a context
                        // so it is safe to reference data
                        let iter: DynTrustIter<'a> =
                            unsafe { std::mem::transmute(vec.titer().unwrap()) };
                        Ok(iter)
                    }
                }
            }
            Data::Scalar(scalar) => {
                match Arc::try_unwrap(scalar) {
                    Ok(scalar) => Ok(scalar.into_titer().unwrap()),
                    Err(scalar) => {
                        // the data is still shared
                        // this should only happen when the data is stored in a context
                        // so it is safe to reference data
                        let iter: DynTrustIter<'a> =
                            unsafe { std::mem::transmute(scalar.titer().unwrap()) };
                        Ok(iter)
                    }
                }
            }
            // #[cfg(feature = "ndarray")]
            Data::Array(array) => {
                if array.ndim() <= 1 {
                    match Arc::try_unwrap(array) {
                        Ok(array) => Ok(array.into_titer().unwrap()),
                        Err(array) => {
                            // the data is still shared
                            // this should only happen when the data is stored in a context
                            // so it is safe to reference data
                            let iter: DynTrustIter<'a> =
                                unsafe { std::mem::transmute(array.titer().unwrap()) };
                            Ok(iter)
                        }
                    }
                } else {
                    Err(array.into())
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
    pub fn new<D: Into<Data<'a>>>(d: D) -> Self {
        Context {
            data: vec![d.into()],
        }
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
