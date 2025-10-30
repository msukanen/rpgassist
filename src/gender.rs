use dicebag::DiceExt;
use serde::{Deserialize, Deserializer, Serialize};

/// Genders, obviously …
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
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

fn bias10_filter<'de, D>(deserializer: D) -> Result<u32, D::Error>
where D: Deserializer<'de> {
    let v = u32::deserialize(deserializer)?;
    Ok(v.min(10))
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Bias10 {
    #[serde(deserialize_with = "bias10_filter")]
    value: u32,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum GenderBias {
    Male(Bias10),
    Female(Bias10),
    None
}

impl Default for GenderBias {
    fn default() -> Self {
        Self::None
    }
}

pub trait HasGenderBias {
    fn gender_bias(&self) -> GenderBias;
}

impl Gender {
    /// Generate a random gender, with or without bias toward one or the other.
    pub fn new(bias: GenderBias) -> Self {
        if 1.d20() +
        match bias {
            GenderBias::Male(v) => -(v.value as i32),
            GenderBias::Female(v) => v.value as i32,
            _ => 0
        } <= 10 { Self::Male }
        else { Self::Female }
    }
}

impl From<&str> for Gender {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "m"|"male"|"mies" => Gender::Male,
            "f"|"n"|"female"|"nainen"|"t"|"tyttö"|"tytto" => Gender::Female,
            _ => unimplemented!("No such gender as '{value}' defined.")
        }
    }
}

impl From<Option<String>> for Gender {
    fn from(value: Option<String>) -> Self {
        if let Some(value) = value {
            Self::from(value.as_str())
        } else {
            Gender::Unspecified
        }
    }
}

pub trait HasGender {
    fn gender(&self) -> Gender;
}