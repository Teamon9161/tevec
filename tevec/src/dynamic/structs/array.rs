#![allow(unreachable_patterns)]
use crate::prelude::*;
use derive_more::From;
use tea_core::ndarray::{
    Array, ArrayBase, ArrayD, ArrayView1, ArrayViewD, ArrayViewMut1, ArrayViewMutD, Axis, Data,
    DataMut, Dimension, Ix0, Ix1, ShapeBuilder, Zip,
};
use tea_macros::GetDtype;

pub trait NdArrayExt<T, D: Dimension> {
    fn apply_along_axis<'a, 'b, S2, T2, F>(
        &'a self,
        out: &'b mut ArrayBase<S2, D>,
        axis: Axis,
        par: bool,
        f: F,
    ) where
        T: Send + Sync + 'a,
        T2: Send + Sync + 'b,
        S2: DataMut<Elem = T2>,
        F: Fn(ArrayView1<'a, T>, ArrayViewMut1<'b, T2>) + Send + Sync;

    fn calc_map_trust_iter_func<'a, F, U: Send + Sync + Clone>(
        &'a self,
        f: F,
        axis: Option<usize>,
        par: Option<bool>,
    ) -> ArrayD<U>
    where
        T: 'a,
        F: Fn(ArrayView1<'a, T>) -> Box<dyn TrustedLen<Item = U> + 'a> + Send + Sync;
}

impl<T: Send + Sync, S: Data<Elem = T>, D: Dimension> NdArrayExt<T, D> for ArrayBase<S, D> {
    fn apply_along_axis<'a, 'b, S2, T2, F>(
        &'a self,
        out: &'b mut ArrayBase<S2, D>,
        axis: Axis,
        par: bool,
        f: F,
    ) where
        T: Send + Sync + 'a,
        T2: Send + Sync + 'b,
        S2: DataMut<Elem = T2>,
        F: Fn(ArrayView1<'a, T>, ArrayViewMut1<'b, T2>) + Send + Sync,
    {
        if self.is_empty() || self.len_of(axis) == 0 {
            return;
        }
        let ndim = self.ndim();
        if ndim == 1 {
            let view = self.view().into_dimensionality::<Ix1>().unwrap();
            f(view, out.view_mut().into_dimensionality::<Ix1>().unwrap());
            return;
        }
        let arr_zip = Zip::from(self.lanes(axis)).and(out.lanes_mut(axis));
        if !par || (ndim == 1) {
            // non-parallel
            arr_zip.for_each(f);
        } else {
            // parallel
            arr_zip.par_for_each(f);
        }
    }

    fn calc_map_trust_iter_func<'a, F, U: Send + Sync + Clone>(
        &'a self,
        f: F,
        axis: Option<usize>,
        par: Option<bool>,
    ) -> ArrayD<U>
    where
        T: 'a,
        F: Fn(ArrayView1<'a, T>) -> Box<dyn TrustedLen<Item = U> + 'a> + Send + Sync,
    {
        let axis = axis.unwrap_or(0);
        let par = par.unwrap_or(false);
        let f_flag = self.is_standard_layout();
        let shape = self.raw_dim().into_shape().set_f(f_flag);
        let mut out_arr = Array::<U, D>::uninit(shape);
        let mut out_wr = out_arr.view_mut();
        let axis = Axis(axis);
        if self.len_of(axis) == 0 {
            // we don't need to do anything
        } else {
            // we don't need a fast path for dim1, as
            // dim1 will use iterator directly
            self.apply_along_axis(&mut out_wr, axis, par, move |x_1d, mut out_1d| {
                let iter = f(x_1d);
                out_1d.write_trust_iter(iter).unwrap();
            });
        }
        unsafe { out_arr.assume_init() }
            .into_dimensionality()
            .unwrap()
    }
}

impl<'a, T, U: 'a> TransmuteDtype<U> for ArbArray<'a, T> {
    type Output = ArbArray<'a, U>;

    #[inline]
    /// # Safety
    ///
    /// the caller must ensure T and U is actually the same type
    unsafe fn into_dtype(self) -> Self::Output {
        std::mem::transmute(self)
    }
}

#[derive(From)]
pub enum ArbArray<'a, T> {
    Owned(ArrayD<T>),
    View(ArrayViewD<'a, T>),
    ViewMut(ArrayViewMutD<'a, T>),
}

#[derive(From, GetDtype)]
pub enum DynArray<'a> {
    Bool(ArbArray<'a, bool>),
    F32(ArbArray<'a, f32>),
    F64(ArbArray<'a, f64>),
    I32(ArbArray<'a, i32>),
    I64(ArbArray<'a, i64>),
    U8(ArbArray<'a, u8>),
    U64(ArbArray<'a, u64>),
    Usize(ArbArray<'a, usize>),
    String(ArbArray<'a, String>),
    OptUsize(ArbArray<'a, Option<usize>>),
    VecUsize(ArbArray<'a, Vec<usize>>),
    #[cfg(feature = "time")]
    DateTime(ArbArray<'a, DateTime>),
    #[cfg(feature = "time")]
    TimeDelta(ArbArray<'a, TimeDelta>),
}
macro_rules! impl_from {

    ($($(#[$meta:meta])? ($arm: ident, $ty: ty, $func_name: ident)),* $(,)?) => {
        impl<'a> DynArray<'a> {
            $(
                $(#[$meta])?
                pub fn $func_name(self) -> TResult<ArbArray<'a, $ty>> {
                    if let DynArray::$arm(v) = self {
                        Ok(v)
                    } else {
                        tbail!("DynArray is not of type {:?}", DataType::$arm)
                    }
            })*
        }

        impl<'a, T: GetDataType + 'a> From<ArrayD<T>> for DynArray<'a> {
            #[allow(unreachable_patterns)]
            #[inline]
            fn from(a: ArrayD<T>) -> Self {
                match T::dtype() {
                    $(
                        $(#[$meta])? DataType::$arm => {
                            // safety: we have checked the type
                            let a: ArbArray<'a, _> = a.into();
                            unsafe{DynArray::$arm(a.into_dtype().into())}
                        },
                    )*
                    type_ => unimplemented!("Create DynArray from type {:?} is not implemented", type_),
                }
            }
        }

        impl<'a, T: GetDataType + 'a> From<ArrayViewD<'a, T>> for DynArray<'a> {
            #[allow(unreachable_patterns)]
            #[inline]
            fn from(a: ArrayViewD<'a, T>) -> Self {
                match T::dtype() {
                    $(
                        $(#[$meta])? DataType::$arm => {
                            // safety: we have checked the type
                            let a: ArbArray<'a, _> = a.into();
                            unsafe{DynArray::$arm(a.into_dtype().into())}
                        },
                    )*
                    type_ => unimplemented!("Create DynArray from type {:?} is not implemented", type_),
                }
            }
        }

        impl<'a, T: GetDataType + 'a> From<ArrayViewMutD<'a, T>> for DynArray<'a> {
            #[allow(unreachable_patterns)]
            #[inline]
            fn from(a: ArrayViewMutD<'a, T>) -> Self {
                match T::dtype() {
                    $(
                        $(#[$meta])? DataType::$arm => {
                            // safety: we have checked the type
                            let a: ArbArray<'a, _> = a.into();
                            unsafe{DynArray::$arm(a.into_dtype().into())}
                        },
                    )*
                    type_ => unimplemented!("Create DynArray from type {:?} is not implemented", type_),
                }
            }
        }
    };
}

impl_from!(
    (Bool, bool, bool),
    (F32, f32, f32),
    (F64, f64, f64),
    (I32, i32, i32),
    (I64, i64, i64),
    (U8, u8, u8),
    (U64, u64, u64),
    (Usize, usize, usize),
    (String, String, string),
    (OptUsize, Option<usize>, opt_usize),
    (VecUsize, Vec<usize>, vec_usize),
    #[cfg(feature = "time")]
    (DateTime, DateTime, datetime),
    #[cfg(feature = "time")]
    (TimeDelta, TimeDelta, timedelta)
);

#[macro_export]
macro_rules! match_array {
    ($($tt: tt)*) => {
        $crate::match_enum!(DynArray, $($tt)*)
    };
}

#[macro_export]
macro_rules! match_arb {
    ($($tt: tt)*) => {
        $crate::match_enum!(ArbArray, $($tt)*)
    };
}

#[macro_export]
/// create dynamic array of dim1
macro_rules! d1_array {
    ($($tt: tt)*) => {
        {
            let vec: DynArray = $crate::ndarray::arr1(& [$($tt)*]).into_dimensionality::<$crate::ndarray::IxDyn>().unwrap().into();
            vec
        }
    };
}

#[macro_export]
/// create dynamic array of dim2
macro_rules! d2_array {
    ($($tt: tt)*) => {
        {
            let vec: DynArray = $crate::ndarray::arr2(& [$($tt)*]).into_dimensionality::<$crate::ndarray::IxDyn>().unwrap().into();
            vec
        }
    };
}

impl<'a, T: Clone> ArbArray<'a, T> {
    #[inline]
    pub fn len(&self) -> usize {
        match_arb!(self; Owned(v) | View(v) | ViewMut(v) => Ok(v.len()),).unwrap()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn ndim(&self) -> usize {
        match_arb!(self; Owned(v) | View(v) | ViewMut(v) => Ok(v.ndim()),).unwrap()
    }

    #[inline]
    pub fn get(&self, index: usize) -> TResult<&T> {
        match_arb!(self; Owned(v) | View(v) | ViewMut(v) => v.get(index).ok_or_else(|| terr!(io(index, v.len()))),)
    }

    #[inline]
    pub fn view(&self) -> ArrayViewD<'_, T> {
        match_arb!(self; Owned(v) | View(v) | ViewMut(v) => Ok(v.view()),).unwrap()
    }

    #[inline]
    pub fn titer<'b>(&'b self) -> TResult<Box<dyn TrustedLen<Item = T> + 'b>> {
        if self.ndim() == 1 {
            match_arb!(self; Owned(v) | View(v) | ViewMut(v) => {
                let v = v.view().into_dimensionality::<Ix1>().unwrap();
                let iter: Box<dyn TrustedLen<Item = T>> = Box::new(v.titer());
                // this is safe as data lives longer than 'a, and 'a is longer than 'b
                // and drop v will not drop the data in memory
                let iter: Box<dyn TrustedLen<Item = T> + 'b> = unsafe { std::mem::transmute(iter) };
                Ok(iter)
            },)
        } else if self.ndim() == 0 {
            match_arb!(self; Owned(v) | View(v) | ViewMut(v) => {
                let scalar = v.view().into_dimensionality::<Ix0>().unwrap().into_scalar();
                Ok(Box::new(std::iter::once(scalar.clone())))
            },)
        } else {
            tbail!("Array with ndim > 1 cannot be converted into iterator")
        }
    }

    #[inline]
    pub fn into_titer(self) -> TResult<Box<dyn TrustedLen<Item = T> + 'a>> {
        if self.ndim() == 1 {
            match_arb!(self;
                Owned(v) => {
                    let len = v.len();
                    Ok(Box::new(v.into_iter().to_trust(len)))
                },
                View(v) => {
                    let len = v.len();
                    Ok(Box::new(v.into_iter().cloned().to_trust(len)))
                },
                ViewMut(v) => {
                    let len = v.len();
                    // TODO: maybe we can use mem::take here? will it be faster?
                    Ok(Box::new(v.into_iter().map(|v| v.clone()).to_trust(len)))
                },
            )
        } else if self.ndim() == 0 {
            match_arb!(self;
                Owned(v) => {
                    let scalar = v.into_dimensionality::<Ix0>().unwrap().into_scalar();
                    Ok(Box::new(std::iter::once(scalar.clone())))
                },
                View(v) | ViewMut(v) => {
                    // TODO: can view mut case can be optimized?
                    // may be faster in case where scalar is a vec or something large
                    let scalar = v.view().into_dimensionality::<Ix0>().unwrap().into_scalar();
                    Ok(Box::new(std::iter::once(scalar.clone())))
                },
            )
        } else {
            tbail!("Array with ndim > 1 cannot be converted into iterator")
        }
    }
}

impl<'a> DynArray<'a> {
    #[inline]
    pub fn len(&self) -> usize {
        match_array!(self; dynamic(v) => Ok(v.len()),).unwrap()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn ndim(&self) -> usize {
        match_array!(self; dynamic(v) => Ok(v.ndim()),).unwrap()
    }

    #[inline]
    #[allow(clippy::clone_on_copy)]
    pub fn get(&self, index: usize) -> TResult<Scalar> {
        match_array!(self; dynamic(v) => v.get(index).map(|v| v.clone().into()),)
    }

    #[inline]
    pub fn view(&self) -> DynArray<'_> {
        match_array!(self; dynamic(v) => Ok(v.view().into()),).unwrap()
    }

    #[inline]
    pub fn titer(&self) -> TResult<DynTrustIter> {
        match_array!(self; dynamic(v) => Ok(v.titer()?.into()),)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn into_titer(self) -> TResult<DynTrustIter<'a>> {
        match_array!(self; dynamic(v) => Ok(v.into_titer()?.into()),)
    }
}
