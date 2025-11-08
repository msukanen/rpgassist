//! Generic `type_name()` accessor(s) for e.g. `log::debug!()` use, etc.
use std::any::type_name;
pub trait GetTypeName {
    fn type_name(&self) -> &'static str;
}

impl <T: ?Sized> GetTypeName for T {
    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}