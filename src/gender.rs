use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Genders, obviously …
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Gender {
    Unspecified,
    Male,
    Female,
}

impl PartialOrd for Gender {
    /// All genders (or lack of such) are treated equal.
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl Gender {
    /// Generate a random gender, with or without bias toward one or the other.
    pub fn new(bias: Option<Gender>) -> Self {
        if 1.d20() +
        match bias {
            Some(Self::Male) => -2,
            Some(Self::Female) => 2,
            _ => 0
        } <= 10 { Self::Male }
        else { Self::Female }
    }
}

pub trait GenderedDisplay {
    fn display_gendered(&self, gender: &Gender) -> std::fmt::Result;
}
