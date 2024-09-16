mod backends_impl;

use tea_core::prelude::{Vec1, Vec1View};
use tea_dtype::{DataType, GetDataType, IsNone};
use tea_error::TResult;

/// Trait for types that can be converted into a dynamic vector representation.
pub trait IntoDyn {
    /// The dynamic vector type that this type can be converted into.
    type Dyn: DynVec1;

    /// Converts this type into its dynamic vector representation.
    fn into_dyn(self) -> Self::Dyn;
}

/// Trait for dynamic vector types that can hold various data types.
pub trait DynVec1 {
    /// The type used for boolean items in the vector.
    type BoolItem: IsNone<Inner = bool>;
    /// The type used for 64-bit floating point items in the vector.
    type F64Item: IsNone<Inner = f64>;
    /// The type used for 32-bit floating point items in the vector.
    type F32Item: IsNone<Inner = f32>;
    /// The type used for 64-bit integer items in the vector.
    type I64Item: IsNone<Inner = i64>;
    /// The type used for 32-bit integer items in the vector.
    type I32Item: IsNone<Inner = i32>;
    type StrItem<'a>: IsNone<Inner = &'a str>;

    /// The vector type for boolean items.
    type BoolVec: Vec1<Self::BoolItem> + IntoDyn<Dyn = Self>;
    /// The vector type for 64-bit floating point items.
    type F64Vec: Vec1<Self::F64Item> + IntoDyn<Dyn = Self>;
    /// The vector type for 32-bit floating point items.
    type F32Vec: Vec1<Self::F32Item> + IntoDyn<Dyn = Self>;
    /// The vector type for 64-bit integer items.
    type I64Vec: Vec1<Self::I64Item> + IntoDyn<Dyn = Self>;
    /// The vector type for 32-bit integer items.
    type I32Vec: Vec1<Self::I32Item> + IntoDyn<Dyn = Self>;
    // type StrVec<'a>: Vec1<Self::StrItem<'a>> + IntoDyn<Dyn = Self>;
    /// Returns the data type of the vector.
    fn get_dtype(&self) -> DataType;
    /// Casts the vector to a new data type.
    fn cast_into(self, dtype: DataType) -> TResult<Self>
    where
        Self: Sized;

    /// Extracts a view of the vector as boolean items.
    fn extract_bool(&self) -> TResult<impl Vec1View<Self::BoolItem>>;

    /// Extracts a view of the vector as 64-bit floating point items.
    fn extract_f64(&self) -> TResult<impl Vec1View<Self::F64Item>>;

    /// Extracts a view of the vector as 32-bit floating point items.
    fn extract_f32(&self) -> TResult<impl Vec1View<Self::F32Item>>;

    /// Extracts a view of the vector as 64-bit integer items.
    fn extract_i64(&self) -> TResult<impl Vec1View<Self::I64Item>>;

    /// Extracts a view of the vector as 32-bit integer items.
    fn extract_i32(&self) -> TResult<impl Vec1View<Self::I32Item>>;

    /// Returns the name of the vector, if available.
    #[inline]
    fn get_name(&self) -> Option<&str> {
        None
    }

    /// Renames the vector and returns a mutable reference to self.
    #[inline]
    fn rename(&mut self, _name: impl AsRef<str>) -> &mut Self {
        self
    }

    /// Returns a new vector with the given name.
    #[inline]
    fn with_name(mut self, name: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.rename(name);
        self
    }

    /// Casts the vector to a new type T.
    #[inline]
    fn cast<T: GetDataType>(self) -> TResult<Self>
    where
        Self: Sized,
    {
        let dtype = self.get_dtype();
        self.cast_into(dtype)
    }
}
