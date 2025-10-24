//! Shapes for various purposes.
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Shape {
    //Animal,// TODO: impl a bunch of animal types?
    AnimalOfChoice,// temporary until Animal above is defined more in depth.
    Bat,
    Claw,
    CrescentMoon,
    Dragon,
    Eagle,
    Fish,
    Hand,
    Hawk,
    Skull,
    Sword,
}

impl Shape {
    pub fn new() -> Self {
        // T-866
        match 1.d10() {
            ..=1 => Self::Dragon,
            2 => Self::Skull,
            3 => Self::Bat,
            4 => Self::Sword,
            5 => Self::Hand,
            6 => Self::CrescentMoon,
            7 => Self::Claw,
            8 => if 1.d2() == 1 {Self::Eagle} else {Self::Hawk},
            9 => Self::Fish,
            _ => Self::AnimalOfChoice
        }
    }
}
