//! 866: Birthmarks
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{body::location::BodyLocation, misc::{color::ExoticColor, shape::Shape}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Birthmark {
    pub location: BodyLocation,
    pub exotic_color: Option<ExoticColor>,
    pub shape: Shape,
}

impl Birthmark {
    /// Generate a random birthmark with location; also color if color is other than "natural".
    // T-866
    pub fn new() -> Self {
        let location = BodyLocation::new();
        let exotic_color = if 1.d20() == 1 {Some(ExoticColor::new())} else {None};
        let shape = Shape::new();

        Self { location, exotic_color, shape }
    }
}
