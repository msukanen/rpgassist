//! Traits to deal with
//! 
//! **a)** "detailed display" of… details.
//! **b)** "proper case" of strings for labels etc.
//! 
//! See: [DetailedDisplay], [ProperCaseExt], [NewsPaperCaseExt]
//! 
/// A trait akin to `.to_string()` etc., but meant for delivery of more
/// detailed information, leaving the other functions to deal with simpler
/// things.
pub trait DetailedDisplay {
    fn detailed_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

/// An extension trait for converting strings into (semi-)"Proper Case."
///
/// This is a "smart" title-casing for "hummie eyes,"
/// suitable for labels, names, and titles. It capitalizes
/// any letter that follows a space, hyphen, or other
/// non-alphabetic character. It doesn't give special
/// treatment to 'words' like "a", "with", and such, however.
///
/// # Example
///
/// ```
/// use rpgassist::ext::ProperCaseExt; // (Adjust path as needed)
///
/// let s = "a test-string (with fawns)";
/// assert_eq!(s.proper_case(), "A Test-String (With Fawns)");
///
/// let s2 = "aDaM cApLiTtEr";
/// assert_eq!(s2.proper_case(), "Adam Caplitter");
/// ```
pub trait ProperCaseExt {
    fn proper_case(&self) -> String;
}

impl ProperCaseExt for str
{
    fn proper_case(&self) -> String {
        let mut output = String::new();
        let mut capitalize_next = true;
        for ch in self.to_lowercase().chars() {
            if capitalize_next && ch.is_alphabetic() {
                output.push_str(&ch.to_uppercase().to_string());
                capitalize_next = false
            } else {
                output.push(ch);
                capitalize_next = !ch.is_alphabetic();
            }
        }
        output
    }
}

/// A trait for functionality that extends on [ProperCaseExt] to deal with
/// special word cases.
/// 
/// TODO: …someday™
pub trait NewsPaperCaseExt : ProperCaseExt {
    fn newspaper_case(&self) -> String;
}