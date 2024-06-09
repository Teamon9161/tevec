mod methods;
mod structs;
#[macro_use]
mod macros;

pub use structs::*;

use tea_dtype::DataType;

pub trait GetDtype {
    fn dtype(&self) -> DataType;
}
