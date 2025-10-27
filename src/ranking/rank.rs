use std::ops::{Add, AddAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::details::DetailedDisplay;

/// Generic "rank" for various things, e.g. stats, skills, etc.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Rank {
    value: i32
}

impl Default for Rank {
    fn default() -> Self {
        Rank { value: 0 }
    }
}

/// A trait for anything with 'rank'.
pub trait IsRanked {
    /// Get current [Rank].
    fn rank(&self) -> Rank;
    /// Get a mutable reference to current [Rank].
    fn rank_mut(&mut self) -> &mut Rank;
}

impl Rank {
    /// Make a new rank.
    fn new(value: i32) -> Self {
        Self { value }
    }

    /// Get a somewhat detailed rank description.
    fn explain(&self) -> &'static str {
        //TODO: add detail to rank explains.
        match &self.value {
            ..=0 => "Rank 0",
            1 => "Rank 1",
            2 => "Rank 2",
            3 => "Rank 3",
            4 => "Rank 4",
            5 => "Rank 5",
            6 => "Rank 6",
            7 => "Rank 7",
            8 => "Rank 8",
            9 => "Rank 9",
            10 => "Rank 10",
            _ => "Rank 11+",
        }
    }

    pub const AVERAGE: Rank = Rank { value: 3 };
    pub const NONE: Rank = Rank { value: 0 };
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl DetailedDisplay for Rank {
    fn detailed_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.explain())
    }
}

impl Add<i32> for Rank {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self { value: self.value + rhs }
    }
}

impl AddAssign<i32> for Rank {
    fn add_assign(&mut self, rhs: i32) {
        self.value += rhs
    }
}

impl Sub<i32> for Rank {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self { value: self.value - rhs }
    }
}

impl SubAssign<i32> for Rank {
    fn sub_assign(&mut self, rhs: i32) {
        self.value -= rhs
    }
}

impl PartialEq<i32> for Rank {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }
}

impl PartialEq<Rank> for i32 {
    fn eq(&self, other: &Rank) -> bool {
        *self == other.value
    }
}

impl PartialOrd<i32> for Rank {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<Rank> for i32 {
    fn partial_cmp(&self, other: &Rank) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl From<i32> for Rank {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod rank_tests {
    use super::*;

    #[test]
    fn add_i32_to_rank() {
        let r = Rank::new(0);
        let r = r + 5;
        assert_eq!(5, r);
    }

    #[test]
    fn sub_i32_from_rank() {
        let r = Rank::new(0);
        let r = r - 5;
        assert_eq!(r, -5);
    }

    #[test]
    fn addassign_i32_to_rank() {
        let mut r = Rank::new(0);
        r += 10;
        assert_eq!(r, 10);
    }

    #[test]
    fn subassign_i32_to_rank() {
        let mut r = Rank::new(0);
        r -= 7;
        assert_eq!(-7, r);
    }
}
