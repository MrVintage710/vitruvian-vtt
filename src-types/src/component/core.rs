use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{ComponentMarker};

//=========================================================================================================================
//           Core Types
//=========================================================================================================================

#[derive(TS, Deserialize, Serialize)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

//=========================================================================================================================
//           Core Components
//=========================================================================================================================


#[derive(TS, Deserialize, Serialize)]
pub struct Name(pub String);
impl ComponentMarker for Name {}

#[derive(TS, Deserialize, Serialize)]
pub struct Damage(pub Dice);
impl ComponentMarker for Damage {}

//=========================================================================================================================
//           Core Components Tests
//=========================================================================================================================

#[cfg(test)]
mod tests {
}