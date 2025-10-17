use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash)]
pub enum Bilateral {
    Left, Right,
}
