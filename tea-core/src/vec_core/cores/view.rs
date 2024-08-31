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
pub trait Vec1View<T>: TIter<T> {
    type SliceOutput<'a>: ToOwned
    where
        Self: 'a,
        // this constraint is needed for ndarray backend
        T: 'a;

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
    fn slice<'a>(&'a self, _start: usize, _end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
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
    unsafe fn uslice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
        self.slice(start, end)
    }

    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> T;

    #[inline(always)]
    fn try_as_slice(&self) -> Option<&[T]> {
        None
    }

    #[inline]
    fn iter_cast<'a, U>(&'a self) -> impl TIterator<Item = U>
    where
        T: 'a + Cast<U>,
    {
        self.titer().map(|v| v.cast())
    }

    #[inline]
    fn opt_iter_cast<'a, U>(&'a self) -> impl TIterator<Item = Option<U>>
    where
        T: IsNone + 'a,
        <T as IsNone>::Inner: Cast<U>,
    {
        self.titer().map(|v| v.to_opt().map(Cast::<U>::cast))
    }

    #[inline]
    fn opt(&self) -> OptIter<Self, T>
    where
        T: IsNone,
        Self: Sized,
    {
        OptIter {
            view: self,
            item: PhantomData,
        }
    }

    #[inline]
    fn to_opt_iter<'a>(&'a self) -> impl TIterator<Item = Option<T::Inner>>
    where
        T: IsNone + 'a,
    {
        self.titer().map(|v| v.to_opt())
    }

    /// if the value is valid, return it, otherwise return None
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<T::Inner>
    where
        T: IsNone,
    {
        self.uget(index).to_opt()
    }

    #[inline]
    fn get(&self, index: usize) -> TResult<T> {
        if index < self.len() {
            Ok(unsafe { self.uget(index) })
        } else {
            tbail!(oob(index, self.len()))
        }
    }

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

    #[inline]
    /// Rolling and apply a custom funtion to each window, but it won't collect result
    fn rolling_custom_iter<'a, U, F>(&'a self, window: usize, mut f: F) -> impl TrustedLen<Item = U>
    where
        F: FnMut(Self::SliceOutput<'a>) -> U,
        T: 'a,
    {
        (1..self.len() + 1)
            .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
            .map(move |(end, start)| f(self.slice(start, end).unwrap()))
            .to_trust(self.len())
    }

    /// Rolling and apply a custom funtion to each window
    #[inline]
    fn rolling_custom<'a, O: Vec1<OT>, OT: Clone, F>(
        &'a self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Self::SliceOutput<'a>) -> OT,
        Self: 'a,
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

    /// Rolling and apply a custom funtion to each window
    ///
    /// Different with `rolling_custom`, the caller should pass a mut reference
    /// of uninit vec.
    /// Be careful to use this function as it will panic in polars backend.
    /// use `rolling_custom` instead
    #[inline]
    fn rolling_custom_to<'a, O: Vec1<OT>, OT, F>(
        &'a self,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        F: FnMut(Self::SliceOutput<'a>) -> OT,
        Self: 'a,
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

    /// Rolling and apply a custom funtion to each window of two vecs
    #[inline]
    fn rolling2_custom<O: Vec1<OT>, OT: Clone, V2, T2, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        V2: Vec1View<T2>,
        F: FnMut(Self::SliceOutput<'_>, V2::SliceOutput<'_>) -> OT,
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

    /// Rolling and apply a function, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window
    #[inline]
    fn rolling_apply<O: Vec1<OT>, OT, F>(
        &self,
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

    /// Rolling and apply a function, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window.
    ///
    /// Different with `rolling_apply`, the caller should pass a mut reference
    /// of uninit vec.
    /// Be careful to use this function as it will panic in polars backend.
    /// use `rolling_apply` instead
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

    /// Rolling and apply a function to both vecs, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window
    #[inline]
    fn rolling2_apply<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
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

    #[inline]
    /// Rolling and apply a function to both vecs, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window.
    ///
    /// Different with `rolling_apply`, the caller should pass a mut reference
    /// of uninit vec.
    /// Be careful to use this function as it will panic in polars backend.
    /// use `rolling_apply` instead
    fn rolling2_apply_to<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
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

    #[inline]
    fn rolling_apply_idx<O: Vec1<OT>, OT, F>(
        &self,
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

    #[inline]
    /// be careful to use this function as it will panic in polars backend.
    /// use rolling_apply_idx instead
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

    #[inline]
    fn rolling2_apply_idx<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
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

    #[inline]
    /// be careful to use this function as it will panic in polars backend.
    /// use rolling2_apply_idx instead
    fn rolling2_apply_idx_to<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        // start, end, value
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

impl<I: TIter<T>, T> TIter<T> for std::sync::Arc<I> {
    #[inline]
    fn titer<'a>(&'a self) -> impl TIterator<Item = T>
    where
        T: 'a,
    {
        (**self).titer()
    }
}

impl<V: Vec1View<T>, T> Vec1View<T> for std::sync::Arc<V> {
    type SliceOutput<'a> = V::SliceOutput<'a>
    where
        Self: 'a,
        T: 'a;

    #[inline]
    fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
        (**self).slice(start, end)
    }

    #[inline]
    unsafe fn uslice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
        (**self).uslice(start, end)
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        (**self).get_backend_name()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        (**self).uget(index)
    }

    #[inline]
    fn try_as_slice(&self) -> Option<&[T]> {
        (**self).try_as_slice()
    }

    #[inline]
    fn rolling_custom<'a, O: Vec1<OT>, OT: Clone, F>(
        &'a self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Self::SliceOutput<'a>) -> OT,
        T: 'a,
    {
        (**self).rolling_custom(window, f, out)
    }

    #[inline]
    fn rolling_apply<O: Vec1<OT>, OT, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        T: Clone,
        F: FnMut(Option<T>, T) -> OT,
    {
        (**self).rolling_apply(window, f, out)
    }

    #[inline]
    fn rolling2_apply<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        T: Clone,
        T2: Clone,
        F: FnMut(Option<(T, T2)>, (T, T2)) -> OT,
    {
        (**self).rolling2_apply(other, window, f, out)
    }
    #[inline]
    fn rolling_apply_idx<O: Vec1<OT>, OT, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, T) -> OT,
    {
        (**self).rolling_apply_idx(window, f, out)
    }

    #[inline]
    fn rolling2_apply_idx<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, (T, T2)) -> OT,
    {
        (**self).rolling2_apply_idx(other, window, f, out)
    }
}
