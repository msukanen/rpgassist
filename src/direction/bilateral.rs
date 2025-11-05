//! Bilateralisms - left/right, front/back, etc.
use std::fmt::Display;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Some bilateralisms…
#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub enum Bilateral {
    Left, Right,
    Front, Back,
}

impl Bilateral {
    /// Generate left/right randomly.
    pub fn random_lr() -> Self {
        match 1.d2() {
            1 => Self::Left,
            _ => Self::Right
        }
    }

    /// Generate front/back randomly.
    pub fn random_fb() -> Self {
        match 1.d2() {
            1 => Self::Front,
            _ => Self::Back
        }
    }
}

impl Display for Bilateral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Back => "back",
            Self::Front => "front",
            Self::Left => "left",
            Self::Right => "right",
        })
    }
}