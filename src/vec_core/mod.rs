mod backends_impl;
mod iter;
mod iter_valid;
pub mod trusted;

use std::marker::PhantomData;
use std::iter::Iterator;
use crate::prelude::IsNone;
use iter::{OwnIter, ViewIter, MutIter};

pub use trusted::TrustedLen;

pub trait VecView1D<T> {

    fn len(&self) -> usize;

    unsafe fn uget(&self, index: usize) -> &T;

    // if the value is valid, return it, otherwise return None
    unsafe fn uvget(&self, index: usize) -> Option<&T> where T: IsNone;

    #[inline]
    fn get(&self, index: usize) -> &T {
        if index < self.len() {
            unsafe { self.uget(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn vget(&self, index: usize) -> Option<&T> where T: IsNone{
        if index < self.len() {
            unsafe { self.uvget(index) }
        } else {
            None
        }
    }

    #[inline]
    fn fold<U, F>(&self, init: U, mut f: F) -> U
    where
        F: FnMut(U, &T) -> U,
    {
        let mut acc = init;
        for i in 0..self.len() {
            let v = unsafe{self.uget(i)};
            acc = f(acc, v);
        };
        acc
    }

    #[inline]
    fn vfold<U, F>(&self, init: U, mut f: F) -> U
    where
        F: FnMut(U, &T) -> U,
        T: IsNone,
    {
        let mut acc = init;
        for i in 0..self.len() {
            let v = unsafe{self.uvget(i)};
            if let Some(v) = v {
                acc = f(acc, v);
            }
        };
        acc
    }

    #[inline]
    fn vfold_n<U, F>(&self, init: U, mut f: F) -> (usize, U)
    where
        F: FnMut(U, &T) -> U,
        T: IsNone,
    {
        let mut acc = init;
        let mut count = 0;
        for i in 0..self.len() {
            let v = unsafe{self.uvget(i)};
            if let Some(v) = v {
                acc = f(acc, v);
                count += 1;
            }
        };
        (count, acc)
    }

    #[inline]
    fn map<U, F, O: Vec1D<U>>(&self, mut f: F) -> O
    where
        F: FnMut(&T) -> U,
    {
        let iter = (0..self.len()).into_iter().map(|i| {
            let v = unsafe{self.uget(i)};
            f(v)
        });
        Vec1D::collect_from_trusted(iter)
    }

    #[inline]
    fn iter_view(&self) -> ViewIter<T, Self> 
    where
    {
        ViewIter {
            idx: 0,
            len: self.len(),
            data: self,
            _element_dtype: PhantomData,
        }
    }
}

pub trait VecMut1D<T>: VecView1D<T> {

    unsafe fn uget_mut(&mut self, index: usize) -> &mut T;

    #[inline]
    fn get_mut(&mut self, index: usize) -> &mut T {
        if index < self.len() {
            unsafe { self.uget_mut(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn iter_mut(&mut self) -> MutIter<T, Self> 
    where
    {
        MutIter {
            idx: 0,
            len: self.len(),
            data: self,
            _element_dtype: PhantomData,
        }
    }

    fn map_inplace(&mut self, mut f: impl FnMut(&mut T)) {
        for i in 0..self.len() {
            let v = unsafe{self.uget_mut(i)};
            f(v);
        }
    }
} 

pub trait Vec1D<T>: VecMut1D<T> {
    
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self;

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T>+TrustedLen>(iter: I) -> Self where Self: Sized {
        Self::collect_from_iter(iter)
    }

    fn into_iter(self) -> OwnIter<T, Self> 
    where Self: Sized
    {
        OwnIter {
            idx: 0,
            len: self.len(),
            data: self,
            _element_dtype: PhantomData,
        }
    }
}

pub trait Vec1DCollect: Iterator {

    #[inline]
    fn collect_vec1d<O: Vec1D<Self::Item>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1D<Self::Item>>::collect_from_iter(self)
    }

    #[inline]
    fn collect_trusted<O: Vec1D<Self::Item>>(self) -> O
    where
        Self: Sized + TrustedLen,
    {
        <O as Vec1D<Self::Item>>::collect_from_trusted(self)
    }
}

impl<T: Iterator + Sized> Vec1DCollect for T {}