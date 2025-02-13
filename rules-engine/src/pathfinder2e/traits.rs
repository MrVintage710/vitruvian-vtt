use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum Trait {
    Barbarian,
    Concentrate,
    Emotion,
    Mental,
    Rage,
    Brawling,
    Primal,
    Morph,
    Grapple,
    Unarmed,
    Agile,
    Fighter,
    Flourish,
    Shove,
    Trip
}