pub trait BoolType {
    fn bool_(self) -> bool;
}

impl BoolType for bool {
    #[inline(always)]
    fn bool_(self) -> bool {
        self
    }
}
