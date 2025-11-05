/// Resolve something or other "in place".
pub trait ResolveInPlace {
    fn resolve(&mut self);
}