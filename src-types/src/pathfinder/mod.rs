use serde::{Deserialize, Serialize};

pub mod class;
pub mod action;
pub mod feat;
pub mod passive;

//=========================================================================================================================
//         Common Types
//=========================================================================================================================

// Attribute
#[derive(Deserialize, Serialize)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

// Attribue Boost
#[derive(Deserialize, Serialize)]
pub enum AttributeBoost {
    Single(Attribute),
    Double(Attribute, Attribute),
    Free
}

// Proficiency
#[derive(Deserialize, Serialize)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
    Mythic
}

// Skills
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery
}

//Trait
#[derive(Debug, Serialize, Deserialize)]
pub enum Trait {
    Barbarian,
    Concentrate,
    Emotion,
    Mental,
    Rage
}