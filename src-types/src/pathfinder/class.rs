use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{action::Action, passive::Passive, skill::Skill, AttributeBoost, Proficiency};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub key_attribute : AttributeBoost,
    pub hit_points : i8,
    pub save_proficiencies : SaveProficiency,
    pub attack_proficiencies : AttackProficiency,
    pub defense_proficiencies : DefenseProficiency,
    pub class_proficiency : Proficiency,
    pub spell_attack_proficiency : Proficiency,
    pub skill_proficiencies : HashMap<Skill, Proficiency>,
    pub class_features_by_level : HashMap<u8, Vec<ClassFeature>>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ClassFeature {
    Multiple {
        name : String,
        description : String, 
        features: Vec<ClassFeature>
    },
    Choice {
        name : String,
        description : String,
        number_of_choices : u8,
        features : Vec<ClassFeature>
    },
    Action {
        name : String,
        description : String,
        action : Action
    },
    Passive {
        name : String,
        description : String,
        passive : Passive
    },
    AttributeBoost {
        number_of_boosts : u8,
    },
}

#[derive(Deserialize, Serialize)]
pub struct SaveProficiency {
    pub fortitude : Proficiency,
    pub reflex : Proficiency,
    pub will : Proficiency,
}

#[derive(Deserialize, Serialize)]
pub struct AttackProficiency {
    pub simple : Proficiency,
    pub martial : Proficiency,
    pub unarmed : Proficiency,
}

#[derive(Deserialize, Serialize)]
pub struct DefenseProficiency {
    pub light_armor : Proficiency,
    pub medium_armor : Proficiency,
    pub heavy_armor : Proficiency,
    pub unarmored : Proficiency,
}