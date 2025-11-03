use dicebag::DiceExt;
use serde::{Deserialize, Deserializer, Serialize};

use crate::resolve::resolve_in_place::ResolveInPlace;

/// Genders, obviously …
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Gender {
    Unspecified,
    Male,
    Female,
    NeverApplicable,
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
/// A bias value/modifier for dice rolls. What the set value actually
/// means, depends on [Bias10]'s usage context itself.
pub struct Bias10 {
    #[serde(deserialize_with = "bias10_filter")]
    value: u32,
}

impl Default for Bias10 {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum GenderBias {
    /// Male bias. The higher the [Bias10], the more likely result will be [Gender::Male].
    Male(Bias10),
    /// Approx ⅔ of rolls will result in male.
    Male23,
    /// Female bias. The higher the [Bias10], the more likely result will be [Gender::Female].
    Female(Bias10),
    /// Approx ⅔ of rolls will result in female.
    Female23,
    /// No bias one way or the other. About 1:1 distribution between gender choices.
    None
}

impl Default for GenderBias {
    fn default() -> Self {
        Self::None// RL distribution is "close enough" to 1:1.
    }
}

pub trait HasGenderBias {
    fn gender_bias(&self) -> GenderBias;
}

impl Gender {
    /// Generate a random gender.
    ///
    /// FYI: Gender distribution is *almost* 1:1 in reality (among humans, that is).
    /// It is ever so slightly female-biased, but the difference is too small to affect
    /// dice rolls.
    pub fn random() -> Self {
        Self::random_biased(GenderBias::None)
    }

    /// Generate a random gender, with or without bias toward one or the other.
    pub fn random_biased(bias: GenderBias) -> Self {
        if 1.d20() +
        match bias {
            GenderBias::Male23 => -3,//-3.333 …
            GenderBias::Male(v) => -(v.value as i32),
            GenderBias::Female23 => 4,//+3.333 …
            GenderBias::Female(v) => v.value as i32,
            _ => 0
        } <= 10 { Self::Male }
        else { Self::Female }
    }

    pub fn get_or_random(&self) -> Self {
        match self {
            Self::Unspecified => Self::random_biased(GenderBias::None),
            _ => *self
        }
    }

    pub fn resolve_biased(&mut self, bias: GenderBias) {
        match self {
            Self::Unspecified => *self = Self::random_biased(bias),
            _ => ()
        }
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

impl Default for Gender {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl ResolveInPlace for Gender {
    fn resolve(&mut self) {
        match self {
            Self::Unspecified => *self = Self::random_biased(GenderBias::None),
            _ => ()
        }
    }
}