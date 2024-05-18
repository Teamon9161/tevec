pub trait BoolType: Copy {
    fn bool_(self) -> bool;
}

impl BoolType for bool {
    #[inline(always)]
    fn bool_(self) -> bool {
        self
    }
}

impl BoolType for &bool {
    #[inline(always)]
    fn bool_(self) -> bool {
        *self
    }
}
