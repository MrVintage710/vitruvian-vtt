use std::collections::HashMap;
use mlua::{FromLua, IntoLua};
use serde::{Deserialize, Serialize};
use crate::pak::{index::PakIndex, item::PakItemSearchable};

use super::{feature::Feature, skill::Skill, source::SourceRef, AttributeBoost, Proficiency};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub name : String,
    #[serde(flatten)]
    pub meta : ClassMeta,
    pub references : HashMap<String, Vec<Feature>>,
    pub class_features : Vec<Feature>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source : Option<SourceRef>,
    pub key_attribute : AttributeBoost,
    pub hit_points : i8,
    pub save_proficiencies : SaveProficiency,
    pub attack_proficiencies : AttackProficiency,
    pub defense_proficiencies : DefenseProficiency,
    pub class_proficiency : Proficiency,
    pub spell_attack_proficiency : Proficiency,
    pub skill_proficiencies : HashMap<Skill, Proficiency>,
    pub number_of_skills : u8,
    pub perception_proficiency : Proficiency,
}

impl PakItemSearchable for ClassMeta {
    fn get_indices(&self) -> Vec<PakIndex> {
        let indices = vec![
            // PakIndex::new("source", self.source.clone()),
            // PakIndex::new("key_attribute", self.key_attribute)
        ];
        indices
    }
}

impl IntoLua for ClassMeta {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let obj = lua.create_table()?;
        obj.set("description", self.description)?;
        obj.set("source", self.source)?;
        obj.set("keyAttribute", self.key_attribute)?;
        obj.set("hitPoints", self.hit_points)?;
        obj.set("saveProficiencies", self.save_proficiencies)?;
        obj.set("attackProficiencies", self.attack_proficiencies)?;
        obj.set("defenseProficiencies", self.defense_proficiencies)?;
        obj.set("classProficiency", self.class_proficiency)?;
        obj.set("skillProficiencies", self.skill_proficiencies)?;
        obj.set("spellAttackAroficiency", self.spell_attack_proficiency)?;
        obj.set("perceptionProficiency", self.perception_proficiency)?;
        obj.set("numberOfSkills", self.number_of_skills)?;
        Ok(mlua::Value::Table(obj))
    }
}

impl FromLua for ClassMeta {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() { return Err(mlua::Error::external("Invalid class meta value")); }
        let table = value.as_table().unwrap();
        let description = table.get("description")?;
        let source = table.get("source")?;
        let key_attribute = table.get("keyAttribute")?;
        let hit_points = table.get("hitPoints")?;
        let save_proficiencies = table.get("saveProficiencies")?;
        let attack_proficiencies = table.get("attackProficiencies")?;
        let defense_proficiencies = table.get("defenseProficiencies")?;
        let class_proficiency = table.get("classProficiency")?;
        let spell_attack_proficiency = table.get("spellAttackProficiency")?;
        let skill_proficiencies = table.get("skillProficiencies")?;
        let perception_proficiency = table.get("perceptionProficiency")?;
        let number_of_skills = table.get("numberOfSkills")?;
        Ok(ClassMeta { 
            description, 
            source, 
            key_attribute, 
            hit_points, 
            save_proficiencies, 
            attack_proficiencies, 
            defense_proficiencies, 
            class_proficiency, 
            spell_attack_proficiency, 
            skill_proficiencies, 
            perception_proficiency,
            number_of_skills
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SaveProficiency {
    pub fortitude : Proficiency,
    pub reflex : Proficiency,
    pub will : Proficiency,
}

impl IntoLua for SaveProficiency {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let obj = lua.create_table()?;
        obj.set("fortitude", self.fortitude)?;
        obj.set("reflex", self.reflex)?;
        obj.set("will", self.will)?;
        Ok(mlua::Value::Table(obj))
    }
}

impl FromLua for SaveProficiency {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() { return Err(mlua::Error::external("Invalid save proficiency value")); }
        let table = value.as_table().unwrap();
        let fortitude = table.get("fortitude")?;
        let reflex = table.get("reflex")?;
        let will = table.get("will")?;
        Ok(SaveProficiency { fortitude, reflex, will })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AttackProficiency {
    pub simple : Proficiency,
    pub martial : Proficiency,
    pub advanced : Proficiency,
    pub unarmed : Proficiency,
}

impl IntoLua for AttackProficiency {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let obj = lua.create_table()?;
        obj.set("simple", self.simple)?;
        obj.set("martial", self.martial)?;
        obj.set("unarmed", self.unarmed)?;
        Ok(mlua::Value::Table(obj))
    }
}

impl FromLua for AttackProficiency {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() { return Err(mlua::Error::external("Invalid attack proficiency value")); }
        let table = value.as_table().unwrap();
        let simple = table.get("simple")?;
        let martial = table.get("martial")?;
        let advanced = table.get("advanced")?;
        let unarmed = table.get("unarmed")?;
        Ok(AttackProficiency { simple, martial, advanced, unarmed })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefenseProficiency {
    pub light_armor : Proficiency,
    pub medium_armor : Proficiency,
    pub heavy_armor : Proficiency,
    pub unarmored : Proficiency,
}

impl IntoLua for DefenseProficiency {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let obj = lua.create_table()?;
        obj.set("lightArmor", self.light_armor)?;
        obj.set("mediumArmor", self.medium_armor)?;
        obj.set("heavyArmor", self.heavy_armor)?;
        obj.set("unarmored", self.unarmored)?;
        Ok(mlua::Value::Table(obj))
    }
}

impl FromLua for DefenseProficiency {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() { return Err(mlua::Error::external("Invalid defense proficiency value")); }
        let table = value.as_table().unwrap();
        let light_armor = table.get("lightArmor")?;
        let medium_armor = table.get("mediumArmor")?;
        let heavy_armor = table.get("heavyArmor")?;
        let unarmored = table.get("unarmored")?;
        Ok(DefenseProficiency { light_armor, medium_armor, heavy_armor, unarmored })
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClassRefereces {
    #[serde(flatten)]
    pub references : HashMap<String, Vec<Feature>>
}