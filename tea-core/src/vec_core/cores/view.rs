use std::marker::PhantomData;

use tea_dtype::{Cast, IsNone};
use tea_error::{tbail, TResult};

use super::super::iter::{OptIter, TIter};
use super::super::iter_traits::TIterator;
use super::super::uninit::UninitRefMut;
use super::own::{Vec1, Vec1Collect};
use crate::prelude::{ToTrustIter, TrustedLen, WriteTrustIter};

/// A trait representing a view into a vector-like structure.
///
/// This trait provides methods for accessing and manipulating data in a vector-like structure
/// without necessarily owning the data. It allows for operations such as slicing, indexing,
/// and iterating over the elements.
///
/// The trait is generic over the type `T`, which represents the type of elements stored in the vector.
///
/// Implementations of this trait should provide efficient access to the underlying data
/// and support various operations that are common for vector-like structures.
///
/// # Type Parameters
///
/// * `T`: The type of elements stored in the vector-like structure.
///
/// # Associated Types
///
/// * `SliceOutput<'a>`: The type returned by the `slice` method. This type must implement `ToOwned`
///   to allow conversion to an owned type if needed.
///
/// Implementations of this trait might include views into contiguous memory arrays,
/// database columns, or other data structures that can be logically viewed as a vector.
pub trait Vec1View<'a, T>: TIter<'a, T> {
    type SliceOutput<'s>: ToOwned
    where
        Self: 's,
        // this constraint is needed for ndarray backend
        T: 's;

    /// Returns the name of the backend implementation.
    ///
    /// This method is useful for debugging and logging purposes, allowing
    /// identification of which specific backend is being used at runtime.
    ///
    /// Additionally, this method can be used to implement backend-specific
    /// optimizations or behaviors in certain methods. By checking the backend
    /// name, code can be written to take advantage of specific backend
    /// capabilities or work around limitations.
    ///
    /// # Returns
    ///
    /// A static string slice containing the name of the backend.
    fn get_backend_name(&self) -> &'static str;

    /// Attempts to create a slice of the vector from the given start and end indices.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting index of the slice.
    /// * `end` - The ending index of the slice (exclusive).
    ///
    /// # Returns
    ///
    /// Returns a `TResult` containing the slice if successful, or an error if slicing is not supported for this backend.
    ///
    /// # Note
    ///
    /// This default implementation returns an error, indicating that slicing is not supported.
    /// Backends that support slicing should override this method.
    #[inline]
    fn slice(&self, _start: usize, _end: usize) -> TResult<Self::SliceOutput<'_>> {
        tbail!("slice is not supported for this backend")
    }

    #[inline]
    /// Creates an unsafe slice of the vector from the given start and end indices.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `start` is less than or equal to `end`.
    /// - `end` is less than or equal to the length of the array.
    /// - The memory range from `start` to `end` is valid and properly initialized.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting index of the slice.
    /// * `end` - The ending index of the slice (exclusive).
    ///
    /// # Returns
    ///
    /// Returns a `TResult<Self::SliceOutput<'a>>` representing the portion of the array from `start` to `end`.
    /// The actual type returned depends on the specific implementation of `Vec1View`.
    unsafe fn uslice(&self, start: usize, end: usize) -> TResult<Self::SliceOutput<'_>> {
        self.slice(start, end)
    }

    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> T;
    /// Attempts to return a reference to the underlying slice of the vector.
    ///
    /// # Returns
    ///
    /// - `Some(&[T])` if the vector's data is contiguous in memory and can be represented as a slice.
    /// - `None` if the vector's data cannot be represented as a contiguous slice.
    ///
    /// # Note
    ///
    /// This method is useful for backends that store data in a contiguous memory layout.
    /// It allows for efficient access to the underlying data without copying.
    /// The default implementation returns `None`, indicating that the data is not
    /// available as a contiguous slice. Backends that can provide this should override this method.
    #[inline(always)]
    fn try_as_slice(&self) -> Option<&[T]> {
        None
    }

    /// Creates an iterator that casts each element of the vector to a new type.
    ///
    /// # Type Parameters
    ///
    /// - `U`: The type to cast each element to.
    ///
    /// # Returns
    ///
    /// An iterator over the vector's elements, with each element cast to type `U`.
    ///
    /// # Note
    ///
    /// This method relies on the `Cast` trait being implemented for the conversion from `T` to `U`.
    #[inline]
    fn iter_cast<U>(&'a self) -> impl TIterator<Item = U>
    where
        T: Cast<U>,
    {
        self.titer().map(|v| v.cast())
    }

    /// Creates an iterator that optionally casts each element of the vector to a new type.
    ///
    /// # Type Parameters
    ///
    /// - `U`: The type to cast each element to.
    ///
    /// # Returns
    ///
    /// An iterator over the vector's elements, with each element optionally cast to type `U`.
    ///
    /// # Note
    ///
    /// This method is useful when dealing with vectors that may contain null or invalid values.
    /// It uses the `IsNone` trait to determine if an element should be considered as `None`.
    #[inline]
    fn opt_iter_cast<U>(&'a self) -> impl TIterator<Item = Option<U>>
    where
        T: IsNone,
        <T as IsNone>::Inner: Cast<U>,
    {
        self.titer().map(|v| v.to_opt().map(Cast::<U>::cast))
    }

    /// Creates an `OptIter` for the vector.
    ///
    /// # Returns
    ///
    /// An `OptIter` instance that allows iterating over the vector's elements as `Option` values.
    ///
    /// # Note
    ///
    /// This method is useful for treating the vector's elements as optional values,
    /// which is particularly helpful when dealing with data that may contain null or invalid entries.
    #[inline]
    fn opt(self) -> OptIter<'a, Self, T>
    where
        Self: Sized,
    {
        OptIter {
            view: self,
            life: PhantomData,
            item: PhantomData,
        }
    }

    /// Creates an iterator that converts each element of the vector to an `Option`.
    ///
    /// # Returns
    ///
    /// An iterator over the vector's elements, with each element converted to `Option<T::Inner>`.
    ///
    /// # Note
    ///
    /// This method is useful for explicitly handling potential null or invalid values in the vector.
    /// It relies on the `IsNone` trait to determine how to convert each element to an `Option`.
    #[inline]
    fn to_opt_iter(&'a self) -> impl TIterator<Item = Option<T::Inner>>
    where
        T: IsNone,
    {
        self.titer().map(|v| v.to_opt())
    }

    /// Retrieves the value at the specified index as an `Option`, if it's valid.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is less than the length of the array.
    ///
    /// # Arguments
    ///
    /// * `index`: The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// - `Some(T::Inner)` if the value at the index is valid.
    /// - `None` if the value is invalid or represents a null value.
    ///
    /// # Note
    ///
    /// This method is unsafe because it doesn't perform bounds checking. It's the caller's
    /// responsibility to ensure that the index is valid.
    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<T::Inner>
    where
        T: IsNone,
    {
        self.uget(index).to_opt()
    }

    /// Safely retrieves the value at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index`: The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(T)` if the index is within bounds.
    /// - `Err(TError)` if the index is out of bounds.
    ///
    /// # Note
    ///
    /// This method performs bounds checking and is safe to use.
    #[inline]
    fn get(&self, index: usize) -> TResult<T> {
        if index < self.len() {
            Ok(unsafe { self.uget(index) })
        } else {
            tbail!(oob(index, self.len()))
        }
    }

    /// Safely retrieves the value at the specified index as an `Option`.
    ///
    /// # Arguments
    ///
    /// * `index`: The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// - `Some(T::Inner)` if the index is within bounds and the value is valid.
    /// - `None` if the index is out of bounds or the value is invalid.
    ///
    /// # Note
    ///
    /// This method combines bounds checking with null value handling.
    #[inline]
    fn vget(&self, index: usize) -> Option<T::Inner>
    where
        T: IsNone,
    {
        if index < self.len() {
            unsafe { self.uvget(index) }
        } else {
            None
        }
    }

    /// Applies a custom function to rolling windows of the vector.
    ///
    /// # Type Parameters
    ///
    /// - `U`: The type returned by the custom function for each window.
    /// - `F`: The type of the custom function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The custom function to apply to each window.
    ///
    /// # Returns
    ///
    /// An iterator over the results of applying the custom function to each window.
    ///
    /// # Note
    ///
    /// This method creates an iterator that doesn't collect results, making it memory-efficient
    /// for large datasets or when further processing of the results is needed.
    #[inline]
    fn rolling_custom_iter<U, F>(&'a self, window: usize, mut f: F) -> impl TrustedLen<Item = U>
    where
        F: FnMut(Self::SliceOutput<'_>) -> U,
        T: 'a,
    {
        (1..self.len() + 1)
            .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
            .map(move |(end, start)| f(self.slice(start, end).unwrap()))
            .to_trust(self.len())
    }

    /// Applies a custom function to rolling windows of the vector and collects the results.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `F`: The type of the custom function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The custom function to apply to each window.
    /// * `out`: An optional pre-allocated output buffer.
    ///
    /// # Returns
    ///
    /// - `Some(O)` if `out` is `None`, containing the collected results.
    /// - `None` if `out` is `Some`, in which case the results are written to the provided buffer.
    ///
    /// # Note
    ///
    /// This method allows for efficient in-place computation when an output buffer is provided.
    #[inline]
    fn rolling_custom<O: Vec1<OT>, OT: Clone, F>(
        &'a self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Self::SliceOutput<'_>) -> OT,
        T: 'a,
    {
        let iter = self.rolling_custom_iter(window, f);
        if let Some(mut out) = out {
            iter.write(&mut out).unwrap();
            None
        } else {
            Some(iter.collect_trusted_vec1())
        }
    }

    /// Applies a custom function to rolling windows of the vector and writes the results to a provided buffer.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `F`: The type of the custom function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The custom function to apply to each window.
    /// * `out`: A mutable reference to an uninitialized output buffer.
    ///
    /// # Note
    ///
    /// This method is more efficient than `rolling_custom` when you have a pre-allocated buffer,
    /// but it may panic with certain backends (e.g., Polars). Use `rolling_custom` for a safer alternative.
    #[inline]
    fn rolling_custom_to<O: Vec1<OT>, OT, F>(
        &'a self,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        F: FnMut(Self::SliceOutput<'_>) -> OT,
        T: 'a,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                let slice = self.uslice(0, i + 1).unwrap();
                out.uset(i, f(slice))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe {
                let slice = self.uslice(start, end + 1).unwrap();
                out.uset(end, f(slice))
            }
        }
    }

    /// Applies a custom function to rolling windows of two vectors simultaneously.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `V2`: The type of the second input vector.
    /// - `T2`: The type of elements in the second input vector.
    /// - `F`: The type of the custom function.
    ///
    /// # Arguments
    ///
    /// * `other`: A reference to the second input vector.
    /// * `window`: The size of the rolling window.
    /// * `f`: The custom function to apply to each pair of windows.
    /// * `out`: An optional pre-allocated output buffer.
    ///
    /// # Returns
    ///
    /// - `Some(O)` if `out` is `None`, containing the collected results.
    /// - `None` if `out` is `Some`, in which case the results are written to the provided buffer.
    ///
    /// # Note
    ///
    /// This method is useful for operations that need to consider two vectors simultaneously,
    /// such as computing rolling correlations or differences between two time series.
    #[inline]
    fn rolling2_custom<'b, O: Vec1<OT>, OT: Clone, V2, T2, F>(
        &'a self,
        other: &'b V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        V2: Vec1View<'b, T2>,
        F: FnMut(Self::SliceOutput<'_>, V2::SliceOutput<'_>) -> OT,
        T: 'a,
        T2: 'b,
    {
        let iter = (1..self.len() + 1)
            .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
            .map(|(end, start)| unsafe {
                f(
                    self.uslice(start, end).unwrap(),
                    other.uslice(start, end).unwrap(),
                )
            });
        if let Some(mut out) = out {
            // TODO: maybe we should return a result here?
            iter.write(&mut out).unwrap();
            None
        } else {
            Some(iter.collect_trusted_vec1())
        }
    }

    /// Applies a rolling function that considers both the removal and addition of elements in the window.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes an `Option<T>` (the element being removed, if any)
    ///        and a `T` (the element being added).
    /// * `out`: An optional pre-allocated output buffer.
    ///
    /// # Returns
    ///
    /// - `Some(O)` if `out` is `None`, containing the collected results.
    /// - `None` if `out` is `Some`, in which case the results are written to the provided buffer.
    ///
    /// # Note
    ///
    /// This method is particularly useful for implementing efficient rolling calculations
    /// where you can update the result based on the elements entering and leaving the window,
    /// rather than recomputing from scratch for each window.
    #[inline]
    fn rolling_apply<O: Vec1<OT>, OT, F>(
        &'a self,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        T: Clone,
        F: FnMut(Option<T>, T) -> OT,
    {
        if let Some(out) = out {
            self.rolling_apply_to::<O, _, _>(window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let remove_value_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain(self.titer().map(Some));
            Some(
                remove_value_iter
                    .zip(self.titer())
                    .map(move |(v_remove, v)| f(v_remove, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    /// Applies a rolling function that considers both the removal and addition of elements in the window,
    /// writing results to a provided buffer.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes an `Option<T>` (the element being removed, if any)
    ///        and a `T` (the element being added).
    /// * `out`: A mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Behavior
    ///
    /// This method applies a rolling function to the vector, considering both elements
    /// entering and leaving the window. It writes the results directly to the provided
    /// output buffer.
    ///
    /// # Safety
    ///
    /// This method uses unsafe operations for performance reasons. It assumes that:
    /// - The `window` size is valid (greater than 0 and not larger than the vector's length).
    /// - The `out` buffer has sufficient capacity to store the results.
    ///
    /// # Note
    ///
    /// This method is more efficient than `rolling_apply` when you have a pre-allocated
    /// buffer. However, it may not be supported by all backends (e.g., it will panic
    /// in the Polars backend). For broader compatibility, use `rolling_apply` instead.
    #[inline]
    fn rolling_apply_to<O: Vec1<OT>, OT, F>(
        &self,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        T: Clone,
        F: FnMut(Option<T>, T) -> OT,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                // no value should be removed in the first window
                out.uset(i, f(None, self.uget(i)))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe {
                // new valid value
                let (v_rm, v) = (self.uget(start), self.uget(end));
                out.uset(end, f(Some(v_rm), v))
            }
        }
    }

    /// Applies a rolling function to two vectors simultaneously, considering both
    /// the removal and addition of elements in the window.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `V2`: The type of the second input vector.
    /// - `T2`: The type of elements in the second input vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `other`: A reference to the second input vector.
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes an `Option<(T, T2)>` (the elements being removed, if any)
    ///        and a `(T, T2)` (the elements being added).
    /// * `out`: An optional mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Returns
    ///
    /// - `Some(O)` if `out` is `None`, containing the collected results.
    /// - `None` if `out` is `Some`, in which case the results are written to the provided buffer.
    ///
    /// # Note
    ///
    /// This method is particularly useful for implementing efficient rolling calculations
    /// that depend on two input vectors simultaneously.
    #[inline]
    fn rolling2_apply<'b, O: Vec1<OT>, OT, V2: Vec1View<'b, T2>, T2, F>(
        &'a self,
        other: &'b V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        T: Clone,
        T2: Clone,
        F: FnMut(Option<(T, T2)>, (T, T2)) -> OT,
    {
        if let Some(out) = out {
            self.rolling2_apply_to::<O, _, _, _, _>(other, window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let remove_value_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain(self.titer().zip(other.titer()).map(Some));
            Some(
                remove_value_iter
                    .zip(self.titer().zip(other.titer()))
                    .map(move |(v_remove, v)| f(v_remove, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    /// Applies a rolling function to two vectors simultaneously, writing results to a provided buffer.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `V2`: The type of the second input vector.
    /// - `T2`: The type of elements in the second input vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `other`: A reference to the second input vector.
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes an `Option<(T, T2)>` (the elements being removed, if any)
    ///        and a `(T, T2)` (the elements being added).
    /// * `out`: A mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Safety
    ///
    /// This method uses unsafe operations for performance reasons. It assumes that:
    /// - The `window` size is valid (greater than 0 and not larger than the vector's length).
    /// - The `out` buffer has sufficient capacity to store the results.
    /// - Both input vectors have the same length.
    ///
    /// # Note
    ///
    /// This method is more efficient than `rolling2_apply` when you have a pre-allocated
    /// buffer. However, it may not be supported by all backends (e.g., it will panic
    /// in the Polars backend). For broader compatibility, use `rolling2_apply` instead.
    #[inline]
    fn rolling2_apply_to<'b, O: Vec1<OT>, OT, V2: Vec1View<'b, T2>, T2, F>(
        &'a self,
        other: &'b V2,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        F: FnMut(Option<(T, T2)>, (T, T2)) -> OT,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                // no value should be removed in the first window
                out.uset(i, f(None, (self.uget(i), other.uget(i))))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe {
                // new valid value
                let (v1_rm, v1) = (self.uget(start), self.uget(end));
                let (v2_rm, v2) = (other.uget(start), other.uget(end));
                out.uset(end, f(Some((v1_rm, v2_rm)), (v1, v2)))
            }
        }
    }

    /// Applies a rolling function that considers the index of elements in the window.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes `Option<usize>` (the start index),
    ///        `usize` (the end index), and `T` (the current element).
    /// * `out`: An optional mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Returns
    ///
    /// - `Some(O)` if `out` is `None`, containing the collected results.
    /// - `None` if `out` is `Some`, in which case the results are written to the provided buffer.
    ///
    /// # Note
    ///
    /// This method is useful when the rolling calculation needs to consider the
    /// position of elements within the vector.
    #[inline]
    fn rolling_apply_idx<O: Vec1<OT>, OT, F>(
        &'a self,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, T) -> OT,
    {
        if let Some(out) = out {
            self.rolling_apply_idx_to::<O, _, _>(window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let start_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain((0..self.len()).map(Some)); // this is longer than expect, but start_iter will stop earlier
            Some(
                self.titer()
                    .zip(start_iter)
                    .enumerate()
                    .map(move |(end, (v, start))| f(start, end, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    /// Applies a rolling function that considers the index of elements in the window,
    /// writing results to a provided buffer.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes `Option<usize>` (the start index),
    ///        `usize` (the end index), and `T` (the current element).
    /// * `out`: A mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Safety
    ///
    /// This method uses unsafe operations for performance reasons. It assumes that:
    /// - The `window` size is valid (greater than 0 and not larger than the vector's length).
    /// - The `out` buffer has sufficient capacity to store the results.
    ///
    /// # Note
    ///
    /// This method is more efficient than `rolling_apply_idx` when you have a pre-allocated
    /// buffer. However, it may not be supported by all backends (e.g., it will panic
    /// in the Polars backend). For broader compatibility, use `rolling_apply_idx` instead.
    #[inline]
    fn rolling_apply_idx_to<O: Vec1<OT>, OT, F>(
        &self,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        // start, end, value
        F: FnMut(Option<usize>, usize, T) -> OT,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                // no value should be removed in the first window
                out.uset(i, f(None, i, self.uget(i)))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe { out.uset(end, f(Some(start), end, self.uget(end))) }
        }
    }

    /// Applies a rolling function to two vectors simultaneously, considering the index of elements in the window.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `V2`: The type of the second input vector.
    /// - `T2`: The type of elements in the second input vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `other`: A reference to the second input vector.
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes `Option<usize>` (the start index),
    ///        `usize` (the end index), and `(T, T2)` (the current elements from both vectors).
    /// * `out`: An optional mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Returns
    ///
    /// - `Some(O)` if `out` is `None`, containing the collected results.
    /// - `None` if `out` is `Some`, in which case the results are written to the provided buffer.
    ///
    /// # Note
    ///
    /// This method is useful when the rolling calculation needs to consider both
    /// the position of elements and values from two input vectors simultaneously.
    #[inline]
    fn rolling2_apply_idx<'b, O: Vec1<OT>, OT, V2: Vec1View<'b, T2>, T2, F>(
        &'a self,
        other: &'b V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, (T, T2)) -> OT,
    {
        if let Some(out) = out {
            self.rolling2_apply_idx_to::<O, _, _, _, _>(other, window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let start_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain((0..self.len()).map(Some)); // this is longer than expect, but start_iter will stop earlier
            Some(
                self.titer()
                    .zip(other.titer())
                    .zip(start_iter)
                    .enumerate()
                    .map(move |(end, ((v, v2), start))| f(start, end, (v, v2)))
                    .collect_trusted_vec1(),
            )
        }
    }

    /// Applies a rolling function to two vectors simultaneously, considering the index of elements in the window,
    /// writing results to a provided buffer.
    ///
    /// # Type Parameters
    ///
    /// - `O`: The output vector type.
    /// - `OT`: The type of elements in the output vector.
    /// - `V2`: The type of the second input vector.
    /// - `T2`: The type of elements in the second input vector.
    /// - `F`: The type of the rolling function.
    ///
    /// # Arguments
    ///
    /// * `other`: A reference to the second input vector.
    /// * `window`: The size of the rolling window.
    /// * `f`: The function to apply. It takes `Option<usize>` (the start index of the window),
    ///        `usize` (the end index of the window), and `(T, T2)` (the current elements from both vectors).
    /// * `out`: A mutable reference to an uninitialized buffer to store the results.
    ///
    /// # Behavior
    ///
    /// This method applies a rolling function to two vectors simultaneously, considering both
    /// the position of elements and values from both input vectors. It writes the results directly
    /// to the provided output buffer.
    ///
    /// # Safety
    ///
    /// This method uses unsafe operations for performance reasons. It assumes that:
    /// - The `window` size is valid (greater than 0 and not larger than the vector's length).
    /// - The `out` buffer has sufficient capacity to store the results.
    /// - Both input vectors have the same length.
    ///
    /// # Note
    ///
    /// This method is more efficient than `rolling2_apply_idx` when you have a pre-allocated
    /// buffer. However, it may not be supported by all backends (e.g., it will panic
    /// in the Polars backend). For broader compatibility, use `rolling2_apply_idx` instead.
    #[inline]
    fn rolling2_apply_idx_to<'b, O: Vec1<OT>, OT, V2: Vec1View<'b, T2>, T2, F>(
        &'a self,
        other: &'b V2,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        F: FnMut(Option<usize>, usize, (T, T2)) -> OT,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                // no value should be removed in the first window
                out.uset(i, f(None, i, (self.uget(i), other.uget(i))))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe { out.uset(end, f(Some(start), end, (self.uget(end), other.uget(end)))) }
        }
    }
}
