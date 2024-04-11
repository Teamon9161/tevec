use crate::prelude::*;

macro_rules! impl_vecview1d_for_vec {
    ($($type: ty),*) => {
        $(
            impl<T> VecView1D<T> for $type
            {
                #[inline]
                fn len(&self) -> usize {
                    self.len()
                }

                #[inline]
                unsafe fn uget(&self, index: usize) -> &T {
                    self.get_unchecked(index)
                }
            }
        )*
    };
    (ref $($type: ty),*) => {
        $(
            impl<T> VecView1D<T> for $type
            {
                #[inline]
                fn len(&self) -> usize {
                    (**self).len()
                }

                #[inline]
                unsafe fn uget(&self, index: usize) -> &T {
                    self.get_unchecked(index)
                }
            }
        )*
    };
}

macro_rules! impl_vecmut1d_for_vec {
    ($($type: ty),*) => {
        $(
            impl<T> VecMut1D<T> for $type
            {
                #[inline]
                unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
                    self.get_unchecked_mut(index)
                }
            }
        )*
    };
}
impl_vecview1d_for_vec!(Vec<T>, [T], [&T]);
impl_vecview1d_for_vec!(ref &[T], &mut [T], &Vec<T>, &mut Vec<T>);
impl_vecmut1d_for_vec!(Vec<T>, [T], &mut [T], &mut Vec<T>);

impl<T> Vec1D<T> for Vec<T> {
    #[inline]
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self
    where
        Self: Sized,
    {
        iter.collect_trusted_to_vec()
    }
}

macro_rules! auto_impl_traits {
    (
        $T: ident,
        $(
            $trait: ident => $($type: ty),*;
        )*
    ) => {
        $(
            $(impl<$T> $trait<$T> for $type {})*
        )*
    };
}

auto_impl_traits!(T,
    Vec1DAgg => Vec<T>, [T], &[T], [&T], &Vec<T>, &mut Vec<T>;
);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let view = &data;
        assert_eq!(VecView1D::len(&data), 5);
        assert_eq!(VecView1D::get(&data, 0), &1);
        let sum = VecView1D::iter_view(&view).fold(0, |acc, x| acc + *x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_collect() {
        let data: Vec<_> = (0..5).collect_vec1d();
        assert_eq!(data, vec![0, 1, 2, 3, 4]);
        let data: Vec<_> = (0..5).collect_trusted();
        assert_eq!(data, vec![0, 1, 2, 3, 4])
    }
}
