
pub trait IsNone {
    fn is_none(&self) -> bool;
    #[inline]
    fn is_not_none(&self) -> bool {
        !self.is_none()
    }
}

impl IsNone for f32 {
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nan() || self.is_infinite()
    }
}

impl IsNone for f64 {
    #[inline]
    fn is_none(&self) -> bool {
        self.is_nan() || self.is_infinite()
    }
}

impl IsNone for i32 {
    #[inline(always)]
    fn is_none(&self) -> bool {
        false
    }
}

impl IsNone for i64 {
    #[inline(always)]
    fn is_none(&self) -> bool {
        false
    }
}

impl IsNone for bool {
    #[inline(always)]
    fn is_none(&self) -> bool {
        false
    }
}