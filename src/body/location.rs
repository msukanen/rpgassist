//! 867: Body Locations
use std::fmt::Display;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::direction::bilateral::Bilateral;

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub enum BodyLocation {
    Abdomen,
    Arm(Bilateral),
    Back,
    Buttocks,
    Chest,
    Eye(Bilateral),
    Face,
    Fingers { count: u8, side: Bilateral },
    Foot(Bilateral),
    Genitals,
    Hand(Bilateral),
    Head,
    Leg(Bilateral),
    Thumb(Bilateral),
}

impl BodyLocation {
    /// Generate a random body location.
    /// 
    // Note that not all available [BodyLocation] are included in random generation.
    pub fn random() -> Self {
        // T867
        match 1.d20() {
            ..=1 => Self::Foot(Bilateral::Right),
            2 => Self::Foot(Bilateral::Left),
            3 => Self::Leg(Bilateral::Right),
            4 => Self::Leg(Bilateral::Left),
            5|6 => Self::Abdomen,
            7 => Self::Buttocks,
            8 => if 1.d3() == 1 { Self::Genitals } else { Self::Buttocks },
            9 => Self::Back,
            10..=13 => Self::Chest,
            14 => Self::Arm(Bilateral::Right),
            15 => Self::Arm(Bilateral::Left),
            16 => Self::Hand(Bilateral::Right),
            17 => Self::Hand(Bilateral::Left),
            18 => Self::Head,
            _ => Self::Face
        }
    }
}

impl Display for BodyLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abdomen => write!(f, "abdomen"),
            Self::Arm(lr) => write!(f, "{lr} arm"),
            Self::Back => write!(f, "back"),
            Self::Buttocks => write!(f, "buttocks"),
            Self::Chest => write!(f, "chest"),
            Self::Eye(lr) => write!(f, "{lr} eye"),
            Self::Face => write!(f, "face"),
            Self::Fingers { count, side } => write!(f, "{count} finger{} from {side} hand", if *count!=1 {"s"} else {""}),
            Self::Foot(lr) => write!(f, "{lr} foot"),
            Self::Genitals => write!(f, "genitals"),
            Self::Hand(lr) => write!(f, "{lr} hand"),
            Self::Head => write!(f, "head"),
            Self::Leg(lr) => write!(f, "{lr} leg"),
            Self::Thumb(lr) => write!(f, "{lr} thumb"),
        }
    }
}