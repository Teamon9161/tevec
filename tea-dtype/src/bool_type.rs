/// A trait for types that can be converted to a boolean value.
///
/// This trait is implemented for `bool` and `&bool`, allowing for a consistent
/// interface to obtain a boolean value from these types.
pub trait BoolType: Copy {
    /// Converts the implementing type to a boolean value.
    ///
    /// # Returns
    ///
    /// A `bool` representing the boolean value of the implementing type.
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
