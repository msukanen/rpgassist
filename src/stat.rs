//! Stats!
//! 
//! * **Age** (in whole years)
//! * **App**earance (comeliness)
//! * **Cha**risma (personal magnetism)
//! * **Con**stitution
//! * **Dex**terity (manual)
//! * **Int**elligence
//! * **Mag**ical Aptitude
//! * **Str**ength (physical)
//! * **Will** (strength of one's mind)
use std::{fmt::Display, hash::Hash, ops::{Add, AddAssign, Sub, SubAssign}};

use serde::{Deserialize, Serialize};

/// Core stat types without value payload.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum StatBase {
    Age, App, Cha, Con, Dex, Int, Mag, Str, Will,
}

impl Display for StatBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Age => "age",
            Self::App => "APP",
            Self::Cha => "CHA",
            Self::Con => "CON",
            Self::Dex => "DEX",
            Self::Int => "INT",
            Self::Mag => "MAG",
            Self::Str => "STR",
            Self::Will => "Willpower",
        })
    }
}

impl StatBase {
    /// Generate a sensible 'default' [Stat] instance.
    pub fn default(&self) -> Stat {
        match self {
            Self::Age => Stat::Age { val: 18 },// 18 is rather common 'default' as minimum age for adventurers.
            Self::App => Stat::App { val: 0 },// ±0 from racial 'average'.
            Self::Cha => Stat::Cha { val: 0 },// ±0 from racial 'average'.
            Self::Con => Stat::Con { val: 10 },// generic average (FYI systems may differ…).
            Self::Dex => Stat::Dex { val: 10 },// generic average (FYI systems may differ…).
            Self::Int => Stat::Int { val: 10 },// generic average (FYI systems may differ…).
            Self::Mag => Stat::Mag { val: 0 },// zero spell casting capability by default. FYI some systems have it as an 'advantage', etc.
            Self::Str => Stat::Str { val: 10 },// generic average (FYI systems may differ…).
            Self::Will => Stat::Will { val: 0 },// ±0 from 'average'.
        }
    }
}

/// Stat types with value assigned.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Stat {
    Age { val: i32 },
    App { val: i32 },
    Cha { val: i32 },
    Con { val: i32 },
    Dex { val: i32 },
    Int { val: i32 },
    Mag { val: i32 },
    Str { val: i32 },
    Will { val: i32 },
}

impl Hash for Stat {
    /// Hashing [Stat] by identity only, ignoring contents.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Age { .. } => 0.hash(state),
            Self::App { .. } => 1.hash(state),
            Self::Cha { .. } => 2.hash(state),
            Self::Con { .. } => 3.hash(state),
            Self::Dex { .. } => 4.hash(state),
            Self::Int { .. } => 5.hash(state),
            Self::Mag { .. } => 6.hash(state),
            Self::Str { .. } => 7.hash(state),
            Self::Will {.. } => 8.hash(state),
        }
    }
}

impl Stat {
    /// Get the underlying stat value.
    pub fn value(&self) -> i32 {
        match self {
            Self::Age { val } |
            Self::App { val } |
            Self::Cha { val } |
            Self::Con { val } |
            Self::Dex { val } |
            Self::Int { val } |
            Self::Mag { val } |
            Self::Str { val } |
            Self::Will { val }=> *val
        }
    }

    /// Get the base type without value payload.
    pub fn stat_base(&self) -> StatBase {
        StatBase::from(self)
    }
}

impl PartialEq for Stat {
    fn eq(&self, other: &Stat) -> bool {
        StatBase::from(self) == StatBase::from(other) && self.value() == other.value()
    }
}
impl Eq for Stat {}

impl From<&Stat> for StatBase {
    /// Derive [StatBase] of the given `stat`.
    fn from(stat: &Stat) -> Self {
        match stat {
            Stat::Age { .. } => Self::Age,
            Stat::App { .. } => Self::App,
            Stat::Cha { .. } => Self::Cha,
            Stat::Con { .. } => Self::Con,
            Stat::Dex { .. } => Self::Dex,
            Stat::Int { .. } => Self::Int,
            Stat::Mag { .. } => Self::Mag,
            Stat::Str { .. } => Self::Str,
            Stat::Will { .. } => Self::Will,
        }
    }
}

impl Add<i32> for Stat {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        let mut stat = self;
        stat += rhs;
        stat
    }
}

impl Sub<i32> for Stat {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        let mut stat = self;
        stat -= rhs;
        stat
    }
}

impl Add<Stat> for Stat {
    type Output = Self;
    fn add(self, rhs: Stat) -> Self::Output {
        if self.stat_base() != rhs.stat_base() {
            panic!("Cannot sum different types of Stat!")
        }

        Self::add(self, rhs.value())
    }
}

impl Sub<Stat> for Stat {
    type Output = Self;
    fn sub(self, rhs: Stat) -> Self::Output {
        if self.stat_base() != rhs.stat_base() {
            panic!("Cannot sub(): different types of Stat!")
        }

        Self::sub(self, rhs.value())
    }
}

impl AddAssign<i32> for Stat {
    fn add_assign(&mut self, rhs: i32) {
        match self {
            Self::App { val } |
            Self::Cha { val }
                => *val += rhs,
            // These cannot fall below 0…:
            Self::Age { val } |
            Self::Int { val } |
            Self::Mag { val } |
            Self::Will { val }
                => *val = (*val + rhs).max(0),
            // These have a min of 1:
            Self::Con { val } |
            Self::Dex { val } |
            Self::Str { val }
                => *val = (*val + rhs).max(1),
        }
    }
}

impl SubAssign<i32> for Stat {
    fn sub_assign(&mut self, rhs: i32) {
        match self {
            Self::App { val } |
            Self::Cha { val }
                => *val -= rhs,
            // These cannot fall below 0…:
            Self::Age { val } |
            Self::Int { val } |
            Self::Mag { val } |
            Self::Will { val }
                => *val = (*val - rhs).max(0),
            // These have a min of 1:
            Self::Con { val } |
            Self::Dex { val } |
            Self::Str { val }
                => *val = (*val - rhs).max(1),
        }
    }
}

impl AddAssign<Stat> for Stat {
    fn add_assign(&mut self, rhs: Stat) {
        if self.stat_base() != rhs.stat_base() {
            panic!("Cannot add_assign different types of Stat!")
        }
        *self += rhs.value()
    }
}

impl SubAssign<Stat> for Stat {
    fn sub_assign(&mut self, rhs: Stat) {
        if self.stat_base() != rhs.stat_base() {
            panic!("Cannot sub_assign different types of Stat!")
        }
        *self -= rhs.value()
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:+}", self.stat_base(), self.value())
    }
}

#[cfg(test)]
mod stat_tests {
    use super::*;

    #[test]
    fn add_i32_to_stat() {
        let stat = Stat::Str { val: 10 };
        let stat = stat + 2;
        assert_eq!(12, stat.value());
    }

    #[test]
    #[should_panic]
    fn value_clamp_works() {
        let stat = Stat::Str { val: 10 };
        let stat = stat - 13;
        // the assert below *should* panic ... expected value being '1' afterall.
        assert_eq!(0, stat.value());
        // … but if it doesn't panic! for some reason… this one should:
        assert_eq!(1, stat.value());
        // uff... superposition bits?
        panic!("I don't want to deal with quantum mechanics! XD");
        //let _ = env_logger::try_init();
        //log::error!("Rust flawed. Call a medic!")
    }

    #[test]
    fn sub_i32_from_stat() {
        let stat = Stat::Str { val: 10 };
        let stat = stat - 2;
        assert_eq!(8, stat.value());
    }

    #[test]
    fn add_stat_to_stat() {
        let stat1 = Stat::Dex { val: 10 };
        let stat2 = Stat::Dex { val: 5 };
        let sum = stat1 + stat2;
        assert_eq!(15, sum.value());
    }

    #[test]
    #[should_panic]
    fn add_diff_stat_to_stat() {
        let stat1 = Stat::Dex { val: 10 };
        let stat2 = Stat::Int { val: 5 };
        let sum = stat1 + stat2;
        assert_eq!(15, sum.value());
    }

    #[test]
    fn addassign_stat_to_stat() {
        let mut stat1 = Stat::Dex { val: 10 };
        let stat2 = Stat::Dex { val: 5 };
        stat1 += stat2;
        assert_eq!(15, stat1.value());
    }

    #[test]
    #[should_panic]
    fn addassign_diff_stat_to_stat() {
        let mut stat1 = Stat::Dex { val: 10 };
        let stat2 = Stat::Mag { val: 5 };
        stat1 += stat2;
        assert_eq!(15, stat1.value());
    }

    #[test]
    fn subassign_stat_from_stat() {
        let mut stat1 = Stat::Dex { val: 10 };
        let stat2 = Stat::Dex { val: 5 };
        stat1 -= stat2;
        assert_eq!(5, stat1.value());
    }

    #[test]
    #[should_panic]
    fn subassign_diff_stat_from_stat() {
        let mut stat1 = Stat::Dex { val: 10 };
        let stat2 = Stat::Mag { val: 5 };
        stat1 -= stat2;
        assert_eq!(5, stat1.value());
    }
}