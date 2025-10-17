use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ExoticColor {
    Dark(Box<ExoticColor>),
    Pastel(Box<ExoticColor>),
    // T-865
    Red, Crimson, Scarlet, BloodRed,
    RedOrange, SunsetOrange,
    Orange,
    YellowOrange,
    Yellow,
    YellowGreen, Citrine,
    Green,
    BlueGreen, Aquamarine, Tourquoise,
    Blue,
    BlueViolet, RoyalBlue,
    Violet, Purple, Lavender,
    RedViolet, Magenta, HotPink,
    Pink,
    White, SnowWhite, OffWhite, Ivory,
    Black, Ebony, TrueBlack, VantaBlack,
    Gray,
    Maroon, ReddishBrown, PurplishBrown,
    Silver,
    Gold,
    Platinum,
}

impl ExoticColor {
    /// Generate a random exotic color which may or may not be hued/tinted.
    pub fn new() -> Self {
        let base = match 1.d(19) {
            ..=1 => match 1.d4() {
                ..=1 => Self::Red,
                2 => Self::Crimson,
                3 => Self::Scarlet,
                _ => Self::BloodRed
            },

            2 => if 1.d2() == 1 {Self::RedOrange} else {Self::SunsetOrange},
            3 => Self::Orange,
            4 => Self::YellowOrange,
            5 => Self::Yellow,
            6 => if 1.d2() == 1 {Self::YellowGreen} else {Self::Citrine},
            7 => Self::Green,
            8 => match 1.d3() {
                ..=1 => Self::BlueGreen,
                2 => Self::Aquamarine,
                _ => Self::Tourquoise
            },

            9 => Self::Blue,
            10 => if 1.d2() == 1 {Self::BlueViolet} else {Self::RoyalBlue},
            11 => match 1.d3() {
                ..=1 => Self::Violet,
                2 => Self::Purple,
                _ => Self::Lavender
            },

            12 => match 1.d3() {
                ..=1 => Self::RedViolet,
                2 => Self::Magenta,
                _ => Self::HotPink
            },

            13 => Self::Pink,
            14 => match 1.d4() {
                ..=1 => Self::White,
                2 => Self::SnowWhite,
                3 => Self::OffWhite,
                _ => Self::Ivory
            },

            15 => match 1.d3() {
                ..=1 => Self::Black,
                2 => Self::Ebony,
                _ => Self::TrueBlack
            },

            16 => Self::Gray,
            17 => match 1.d3() {
                ..=1 => Self::Maroon,
                2 => Self::ReddishBrown,
                _ => Self::PurplishBrown
            },

            18 => if 1.d2() == 1 {Self::Silver} else {Self::Platinum},
            _ => Self::Gold,
        };

        // Choose if we have pure base color or a tinted one.
        if 1.d20() == 1 {
            let base = Box::new(base);
            match 1.d2() {
                1 => Self::Dark(base),
                _ => Self::Pastel(base)
            }
        } else {
            base
        }
    }
}
