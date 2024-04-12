use crate::prelude::*;

impl<T: Clone> Vec1View<T> for Vec<T>
{
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        self.get_unchecked(index).clone()
    }
}

impl<'a, T> Vec1View<&'a T> for &'a Vec<T>
{
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> &'a T {
        self.get_unchecked(index)
    }
}

impl<'a, T> Vec1Mut<T> for &'a mut Vec<T>
{

    #[inline]
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
        self.get_unchecked_mut(index)
    }
}

// macro_rules! impl_Vec1View_for_vec {
//     ($($type: ty),*) => {
//         $(
//             impl<T> Vec1View<T> for $type
//             {
//                 #[inline]
//                 fn len(&self) -> usize {
//                     self.len()
//                 }

//                 #[inline]
//                 unsafe fn uget(&self, index: usize) -> T {
//                     self.get_unchecked(index).clone()
//                 }
//             }

//             impl<'a, T> Vec1View<&'a T> for &'a $type
//             {
//                 #[inline]
//                 fn len(&self) -> usize {
//                     (**self).len()
//                 }

//                 #[inline]
//                 unsafe fn uget(&self, index: usize) -> &'a T {
//                     self.get_unchecked(index)
//                 }
//             }
//         )*
//     };
//     // (ref $($type: ty),*) => {
//     //     $(
//     //         impl<'a, T> Vec1View<&'a T> for &'a $type
//     //         {
//     //             #[inline]
//     //             fn len(&self) -> usize {
//     //                 (**self).len()
//     //             }

//     //             #[inline]
//     //             unsafe fn uget(&self, index: usize) -> &'a T {
//     //                 self.get_unchecked(index)
//     //             }
//     //         }
//     //     )*
//     // };
// }

// macro_rules! impl_vecmut1d_for_vec {
//     ($($type: ty),*) => {
//         $(
//             impl<T> Vec1Mut<T> for $type
//             {
//                 #[inline]
//                 unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
//                     self.get_unchecked_mut(index)
//                 }
//             }

//             impl<'a, T> Vec1Mut<&'a mut T> for &mut $type {
//                 #[inline]
//                 unsafe fn uget_mut(&mut self, index: usize) -> &'a mut T {
//                     self.get_unchecked_mut(index)
//                 }
//             }
//         )*
//     };
// }
// impl_Vec1View_for_vec!(Vec<T>);
// // impl_Vec1View_for_vec!(ref [T], Vec::<T>);
// // impl_Vec1View_for_vec!(ref &[T], &mut [T], &Vec<T>, &mut Vec<T>);
// impl_vecmut1d_for_vec!(Vec<T>);

impl<T: Clone> Vec1<T> for Vec<T> {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let view = &data;
        assert_eq!(Vec1View::len(&data), 5);
        assert_eq!(view.get(0), 1);
        // let sum = Vec1View::iter_view(&view).fold(0, |acc, x| acc + *x);
        // assert_eq!(sum, 15);
    }

    // #[test]
    // fn test_collect() {
    //     let data: Vec<_> = (0..5).collect_vec1d();
    //     assert_eq!(data, vec![0, 1, 2, 3, 4]);
    //     let data: Vec<_> = (0..5).collect_trusted();
    //     assert_eq!(data, vec![0, 1, 2, 3, 4]);
    //     let v: Vec<i32> = vec![];
    //     let data: Vec<i32> = Vec1::empty();
    //     assert_eq!(data, v);
    //     let data: Vec<f64> = vec![Some(1.), None, Some(2.)]
    //         .into_iter()
    //         .collect_vec1d_opt();
    //     assert!(data[1].is_nan());
    //     assert_eq!(data[2], 2.)
    // }
}
