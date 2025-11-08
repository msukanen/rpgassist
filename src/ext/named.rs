//! Trait(s) that deal with "names" of things.
/// A trait for anything with a name/nick.
pub trait IsNamed {
    /// Get the name/nick of self.
    fn name(&self) -> &str;
}