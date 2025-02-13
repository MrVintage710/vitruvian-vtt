use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Feat {
    
}

#[derive(Deserialize, Serialize)]
pub enum FeatType {
    Ancestry,
    Class,
    General,
    Skill,
}