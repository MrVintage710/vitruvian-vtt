use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{action::Action, passive::Passive, skill::Skill, source::{self, SourceRef}, AttributeBoost, Proficiency};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub key_attribute : AttributeBoost,
    pub references : ClassRefereces,
    pub hit_points : i8,
    pub save_proficiencies : SaveProficiency,
    pub attack_proficiencies : AttackProficiency,
    pub defense_proficiencies : DefenseProficiency,
    pub class_proficiency : Proficiency,
    pub spell_attack_proficiency : Proficiency,
    pub skill_proficiencies : HashMap<Skill, Proficiency>,
    pub class_features : Vec<Feature>,
}

#[derive(Deserialize, Serialize)]
pub struct FeatureMeta {
    pub name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level : Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source : Option<SourceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisites : Option<String>,
}

impl FeatureMeta {
    pub fn new(name : &str) -> Self {
        FeatureMeta {
            name: name.to_string(),
            description: None,
            level: None,
            source: None,
            prerequisites: None,
        }
    }
    
    pub fn with_desc(mut self, desc : &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    
    pub fn with_level(mut self, level : u8) -> Self {
        self.level = Some(level);
        self
    }
    
    pub fn with_source(mut self, source : SourceRef) -> Self {
        self.source = Some(source);
        self
    }
    
    pub fn with_prerequisites(mut self, prereq : &str) -> Self {
        self.prerequisites = Some(prereq.to_string());
        self
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Feature {
    Multiple {
        #[serde(flatten)]
        meta : FeatureMeta,
        features: Vec<Feature>
    },
    Choice {
        #[serde(flatten)]
        meta : FeatureMeta,
        number_of_choices : u8,
        features : Vec<Feature>
    },
    Action {
        #[serde(flatten)]
        meta : FeatureMeta,
        #[serde(flatten)]
        action : Action
    },
    Passive {
        #[serde(flatten)]
        meta : FeatureMeta,
        #[serde(flatten)]
        passive : Passive
    },
    AttributeBoost {
        level : u8,
        number_of_boosts : u8,
    },
    ReferenceChioce{
        #[serde(flatten)]
        meta : FeatureMeta,
        number_of_choices : u8,
        module : String
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

#[derive(Deserialize, Serialize)]
pub struct ClassRefereces {
    #[serde(flatten)]
    pub references : HashMap<String, Vec<Feature>>
}