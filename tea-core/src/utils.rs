/// define the compensation to use in kahan summation.
#[macro_export]
macro_rules! define_c {
    ($($c: ident),*) => {
        $(let $c = &mut 0.;)*
    }
}

#[macro_export]
macro_rules! define_n {
    ($($n: ident),*) => {
        $(let $n = &mut 0usize;)*
    }
}

/// Fold over the manually unrolled `a` with `f`, this function is copied from ndarray::utils as it
/// is not a public function.
pub fn vec_fold<T, I, F>(mut a: &[T], init: I, f: F) -> T
where
    T: Clone,
    I: Fn() -> T,
    F: Fn(T, T) -> T,
{
    // eightfold unrolled so that calculation can be vectorized
    let mut acc = init();
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) = (
        init(),
        init(),
        init(),
        init(),
        init(),
        init(),
        init(),
        init(),
    );
    while a.len() >= 8 {
        p0 = f(p0, a[0].clone());
        p1 = f(p1, a[1].clone());
        p2 = f(p2, a[2].clone());
        p3 = f(p3, a[3].clone());
        p4 = f(p4, a[4].clone());
        p5 = f(p5, a[5].clone());
        p6 = f(p6, a[6].clone());
        p7 = f(p7, a[7].clone());

        a = &a[8..];
    }
    acc = f(acc.clone(), f(p0, p4));
    acc = f(acc.clone(), f(p1, p5));
    acc = f(acc.clone(), f(p2, p6));
    acc = f(acc.clone(), f(p3, p7));

    // make it clear to the optimizer that this loop is short
    // and can not be autovectorized.
    for (i, x) in a.iter().enumerate() {
        if i >= 7 {
            break;
        }
        acc = f(acc.clone(), x.clone())
    }
    acc
}

/// Fold over the manually unrolled `xs` with `f`, the third argument is the num of
/// the valid elements, if the new value is valid, the function `f` must execute *n = *n+1.
///
/// **Note that** this function only return correct result when the init values and outputs of `f` are valid.
pub fn vec_nfold<T, I, F>(mut a: &[T], init: I, f: F) -> (usize, T)
where
    T: Clone,
    I: Fn() -> T,
    F: Fn(T, T, &mut usize) -> T,
{
    define_n!(n1, n2, n3, n4, n5, n6, n7, n8);
    // eightfold unrolled so that calculation can be vectorized
    let mut acc = init();
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) = (
        init(),
        init(),
        init(),
        init(),
        init(),
        init(),
        init(),
        init(),
    );
    while a.len() >= 8 {
        p0 = f(p0, a[0].clone(), n1);
        p1 = f(p1, a[1].clone(), n2);
        p2 = f(p2, a[2].clone(), n3);
        p3 = f(p3, a[3].clone(), n4);
        p4 = f(p4, a[4].clone(), n5);
        p5 = f(p5, a[5].clone(), n6);
        p6 = f(p6, a[6].clone(), n7);
        p7 = f(p7, a[7].clone(), n8);

        a = &a[8..];
    }
    acc = f(acc.clone(), f(p0, p4, n1), n5);
    acc = f(acc.clone(), f(p1, p5, n2), n6);
    acc = f(acc.clone(), f(p2, p6, n3), n7);
    acc = f(acc.clone(), f(p3, p7, n4), n8);
    // As init is valid, p1-p8 should be valid as well, the output ot `f` is also valid
    // so just minus eight here.
    let n = &mut (*n1 + *n2 + *n3 + *n4 + *n5 + *n6 + *n7 + *n8 - 8);
    // make it clear to the optimizer that this loop is short
    // and can not be autovectorized.
    for (i, x) in a.iter().enumerate() {
        if i >= 7 {
            break;
        }
        acc = f(acc.clone(), x.clone(), n)
    }
    (*n, acc)
}
