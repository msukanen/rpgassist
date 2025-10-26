use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StatBase {
    Age, App, Cha, Con, Dex, Mag, Str
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Stat {
    Age { value: i32 },
    App { value: i32 },
    Cha { value: i32 },
    Con { value: i32 },
    Dex { value: i32 },
    Mag { value: i32 },
    Str { value: i32 },
}

impl Stat {
    pub fn value(&self) -> i32 {
        match self {
            Self::Age { value }|
            Self::App { value }|
            Self::Cha { value }|
            Self::Con { value }|
            Self::Dex { value }|
            Self::Mag { value }|
            Self::Str { value }=> *value
        }
    }
}