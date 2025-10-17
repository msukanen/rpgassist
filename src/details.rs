pub trait DetailedDisplay {
    fn detailed_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
