pub trait DetailedDisplay {
    fn detailed_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

pub trait ProperCaseExt {
    fn proper_case(&self) -> String;
}

impl ProperCaseExt for str {
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