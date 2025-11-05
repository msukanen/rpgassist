//! Natural-join a list of strings.
//! 
//! Instead of producing typical `"A, B, C, D"`, we make e.g. `"A, B, C and D"`.
//! 
use std::fmt::Display;

pub trait NaturalJoin {
    fn natural_join(self) -> String;
}

impl<I> NaturalJoin for I
where
    I: Iterator,
    I::Item: Display,
{
    /// Join stuff into natural language string for hummie eyes.
    fn natural_join(self) -> String {
        // check if more than one entry…
        let mut iter = self.peekable();
        let mut output = String::new();

        // 1st item, if any…
        match iter.next() {
            None => return output,
            Some(fst) => {
                // break early if just single entry
                if iter.peek().is_none() {
                    return fst.to_string();
                }
                output.push_str(&fst.to_string())
            },
        }

        while let Some(curr) = iter.next() {
            if iter.peek().is_none() {
                output.push_str(" and ");
            } else {
                output.push_str(", ");
            }
            output.push_str(&curr.to_string());
        }

        output
    }
}

#[cfg(test)]
mod natural_join_tests {
    use crate::NaturalJoin;

    #[test]
    fn natural_join_nothing() {
        let v: Vec<String> = vec![];
        let s = v.iter().natural_join();
        assert_eq!("", s);
    }

    #[test]
    fn natural_join_single() {
        let a = ["A"];
        let s = a.iter().natural_join();
        assert_eq!("A", s);
    }

    #[test]
    fn natural_join_two() {
        let v = ["A", "B"];
        let s = v.iter().natural_join();
        assert_eq!("A and B", s);
    }

    #[test]
    fn natural_join_three() {
        let v = ["A", "B", "C"];
        let s = v.iter().natural_join();
        assert_eq!("A, B and C", s);
    }
}