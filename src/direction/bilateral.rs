use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub enum Bilateral {
    Left, Right,
}

impl Bilateral {
    /// Generate left/right randomly.
    pub fn left_or_right() -> Self {
        match 1.d2() {
            1 => Self::Left,
            _ => Self::Right
        }
    }
}
