use serde::{Deserialize, Serialize};

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