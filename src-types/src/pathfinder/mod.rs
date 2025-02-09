use serde::{Deserialize, Serialize};

use crate::common::DiceAmount;

pub mod class;
pub mod action;
pub mod feat;
pub mod passive;
pub mod skill;
pub mod traits;
pub mod def;
pub mod source;

//=========================================================================================================================
//         Common Types
//=========================================================================================================================

// Attribute
#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

// Attribue Boost
#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum AttributeBoost {
    Single(Attribute),
    Double(Attribute, Attribute),
    Free
}

// Proficiency
#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
    Mythic
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
    Precision
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Damage(pub DiceAmount, pub DamageType);

impl ToString for Damage {
    fn to_string(&self) -> String {
        format!("{} {:?}", self.0.to_string(), self.1)
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare
}