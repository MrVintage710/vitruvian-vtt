use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct DiceAmount(pub u32, pub Dice);

impl ToString for DiceAmount {
    fn to_string(&self) -> String {
        format!("{}{:?}", self.0, self.1)
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}