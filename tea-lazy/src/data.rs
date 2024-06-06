use std::sync::Arc;

use tevec::prelude::*;

#[derive(Clone)]
pub enum Data<'a> {
    TrustIter(Arc<DynTrustIter<'a>>),
    Vec(Arc<DynVec>),
}

impl<'a> From<DynTrustIter<'a>> for Data<'a> {
    #[inline]
    fn from(iter: DynTrustIter<'a>) -> Self {
        Data::TrustIter(Arc::new(iter))
    }
}

impl<'a> From<Arc<DynTrustIter<'a>>> for Data<'a> {
    #[inline]
    fn from(iter: Arc<DynTrustIter<'a>>) -> Self {
        Data::TrustIter(iter)
    }
}

impl<'a, T: GetDataType + 'a> From<Box<dyn TrustedLen<Item = T> + 'a>> for Data<'a> {
    #[inline]
    fn from(iter: Box<dyn TrustedLen<Item = T> + 'a>) -> Self {
        let iter: DynTrustIter<'a> = iter.into();
        iter.into()
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

impl From<Arc<DynVec>> for Data<'_> {
    #[inline]
    fn from(vec: Arc<DynVec>) -> Self {
        Data::Vec(vec)
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
