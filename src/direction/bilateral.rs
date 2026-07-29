//! Bilateralisms — left/right, front/back, up/down.
use std::fmt::Display;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Some bilateralisms…
#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub enum Bilateral {
    Left, Right,
    Front, Back,
    Up, Down,
}

impl Bilateral {
    /// Generate any [Bilateral] in random.
    pub fn random() -> Self {
        match 1.d6() {
            1 => Self::Back,
            2 => Self::Down,
            3 => Self::Front,
            4 => Self::Left,
            5 => Self::Right,
            _ => Self::Up
        }
    }

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

    /// Generate up/down randomly.
    pub fn random_ud() -> Self {
        match 1.d2() {
            1 => Self::Up,
            _ => Self::Down
        }
    }

    /// Get the direct opposite.
    pub const fn opposite(&self) -> Self {
        match self {
            Self::Back  => Self::Front,
            Self::Down  => Self::Up,
            Self::Front => Self::Back,
            Self::Left  => Self::Right,
            Self::Right => Self::Left,
            Self::Up    => Self::Down,
        }
    }
}

impl Display for Bilateral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Back  => "back",
            Self::Front => "front",
            Self::Left  => "left",
            Self::Right => "right",
            Self::Up    => "up",
            Self::Down  => "down",
        })
    }
}