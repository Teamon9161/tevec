pub trait IsNone {
    fn is_none(&self) -> bool;
    #[inline]
    fn not_none(&self) -> bool {
        !self.is_none()
    }
}

impl IsNone for f32 {
    #[inline]
    #[allow(clippy::eq_op)]
    fn is_none(&self) -> bool {
        self != self
    }

    #[inline]
    #[allow(clippy::eq_op)]
    fn not_none(&self) -> bool {
        self == self
    }
}

impl IsNone for f64 {
    #[inline]
    #[allow(clippy::eq_op)]
    fn is_none(&self) -> bool {
        self != self
    }

    #[inline]
    #[allow(clippy::eq_op)]
    fn not_none(&self) -> bool {
        self == self
    }
}

macro_rules! impl_not_none {
    ($($type: ty),*) => {
        $(
            impl IsNone for $type {
                #[inline]
                #[allow(clippy::eq_op)]
                fn is_none(&self) -> bool {
                    false
                }

                #[inline]
                #[allow(clippy::eq_op)]
                fn not_none(&self) -> bool {
                    true
                }
            }
        )*
    };
}

impl_not_none!(bool, i32, i64, u64, usize);
