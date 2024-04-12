#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
mod features;
pub const EPS: f64 = 1e-14;

pub use features::RollingValidFeature;

use std::iter::Iterator;

pub trait RollingBasic<T>: IntoIterator<Item = T>
where 
    Self: Sized,
    Self::IntoIter: Clone,
    T: Clone,
{      
    fn rolling_apply<F, U>(self, window: usize, mut f: F) -> impl Iterator<Item = U>
    where
        F: FnMut(Option<T>, T) -> U,
    {
        assert!(window > 0, "window must be greater than 0");
        let iter = self.into_iter();
        let remove_value_iter = std::iter::repeat(None)
            .take(window-1)
            .chain(iter.clone().map(|v| Some(v)));
        iter.zip(remove_value_iter).map(move |(v, v_remove)| f(v_remove, v))
    }
}

pub trait RollingValidBasic<T>: IntoIterator<Item = Option<T>>
where 
    Self: Sized,
    Self::IntoIter: Clone,
    Self::Item: Clone,    
{   
    fn rolling_vapply<F, U>(self, window: usize, mut f: F) -> impl Iterator<Item = U>
    where
        F: FnMut(Option<Option<T>>, Option<T>) -> U,
    {

        assert!(window > 0, "window must be greater than 0");
        let iter = self.into_iter();
        let remove_value_iter = std::iter::repeat::<Option<Option<T>>>(None)
            .take(window-1)
            .chain(iter.clone().map(|v| Some(v)));
        iter.zip(remove_value_iter).map(move |(v, v_remove)| f(v_remove, v))
    }
}


impl<T: Clone, I: IntoIterator<Item=T>> RollingBasic<T> for I
where 
    I::IntoIter: Clone,
{}

impl<T: Clone, I: IntoIterator<Item=Option<T>>> RollingValidBasic<T> for I
where 
    I::IntoIter: Clone,
{}
