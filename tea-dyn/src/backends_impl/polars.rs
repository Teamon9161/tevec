use tea_core::prelude::Vec1View;
use tea_deps::polars::prelude::*;
use tea_dtype::DataType;
use tea_error::TResult;

use crate::{DynVec1, IntoDyn};

impl IntoDyn for Series {
    type Dyn = Self;
    #[inline]
    fn into_dyn(self) -> Self::Dyn {
        self
    }
}

impl<T: PolarsDataType> IntoDyn for ChunkedArray<T>
where
    ChunkedArray<T>: IntoSeries,
{
    type Dyn = Series;
    #[inline]
    fn into_dyn(self) -> Self::Dyn {
        self.into_series()
    }
}

impl DynVec1 for Series {
    type BoolItem = Option<bool>;
    type F32Item = Option<f32>;
    type F64Item = Option<f64>;
    type I32Item = Option<i32>;
    type I64Item = Option<i64>;
    type StrItem<'a> = Option<&'a str>;

    type BoolVec = BooleanChunked;
    type F32Vec = Float32Chunked;
    type F64Vec = Float64Chunked;
    type I32Vec = Int32Chunked;
    type I64Vec = Int64Chunked;
    // type StrVec<'a> = StringChunked;

    #[inline]
    fn get_dtype(&self) -> DataType {
        self.dtype().into()
    }

    #[inline]
    fn get_name(&self) -> Option<&str> {
        Some(self.name())
    }

    #[inline]
    fn rename(&mut self, name: impl AsRef<str>) -> &mut Self {
        Series::rename(self, name.as_ref());
        self
    }

    #[inline]
    fn extract_bool(&self) -> TResult<impl Vec1View<Self::BoolItem>> {
        Ok(self.bool()?)
    }

    #[inline]
    fn extract_f64(&self) -> TResult<impl Vec1View<Self::F64Item>> {
        Ok(self.f64()?)
    }

    #[inline]
    fn extract_f32(&self) -> TResult<impl Vec1View<Self::F32Item>> {
        Ok(self.f32()?)
    }

    #[inline]
    fn extract_i64(&self) -> TResult<impl Vec1View<Self::I64Item>> {
        Ok(self.i64()?)
    }

    #[inline]
    fn extract_i32(&self) -> TResult<impl Vec1View<Self::I32Item>> {
        Ok(self.i32()?)
    }

    #[inline]
    fn cast_into(self, dtype: DataType) -> TResult<Self> {
        let self_dtype = self.get_dtype();
        if self_dtype == dtype {
            return Ok(self);
        }
        Ok(Series::cast(&self, &dtype.into())?)
    }
}
