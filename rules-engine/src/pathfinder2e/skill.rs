use mlua::{FromLua, IntoLua};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore(String),
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

impl IntoLua for Skill {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Skill::Acrobatics => Ok(mlua::Value::String(lua.create_string("acrobatics")?)),
            Skill::Arcana => Ok(mlua::Value::String(lua.create_string("arcana")?)),
            Skill::Athletics => Ok(mlua::Value::String(lua.create_string("athletics")?)),
            Skill::Crafting => Ok(mlua::Value::String(lua.create_string("crafting")?)),
            Skill::Deception => Ok(mlua::Value::String(lua.create_string("deception")?)),
            Skill::Diplomacy => Ok(mlua::Value::String(lua.create_string("diplomacy")?)),
            Skill::Intimidation => Ok(mlua::Value::String(lua.create_string("intimidation")?)),
            Skill::Lore(value) => Ok(mlua::Value::String(lua.create_string(format!("lore:{}", value))?)),
            Skill::Medicine => Ok(mlua::Value::String(lua.create_string("medicine")?)),
            Skill::Nature => Ok(mlua::Value::String(lua.create_string("nature")?)),
            Skill::Occultism => Ok(mlua::Value::String(lua.create_string("occultism")?)),
            Skill::Performance => Ok(mlua::Value::String(lua.create_string("performance")?)),
            Skill::Religion => Ok(mlua::Value::String(lua.create_string("religion")?)),
            Skill::Society => Ok(mlua::Value::String(lua.create_string("society")?)),
            Skill::Stealth => Ok(mlua::Value::String(lua.create_string("stealth")?)),
            Skill::Survival => Ok(mlua::Value::String(lua.create_string("survival")?)),
            Skill::Thievery => Ok(mlua::Value::String(lua.create_string("thievery")?)),
        }
    }
}

impl FromLua for Skill {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let s = value.to_string()?;
        if s.starts_with("lore") {
            return Ok(Skill::Lore(s[5..].to_string()));
        }
        match s.as_str() {
            "acrobatics" => Ok(Skill::Acrobatics),
            "arcana" => Ok(Skill::Arcana),
            "athletics" => Ok(Skill::Athletics),
            "crafting" => Ok(Skill::Crafting),
            "deception" => Ok(Skill::Deception),
            "diplomacy" => Ok(Skill::Diplomacy),
            "intimidation" => Ok(Skill::Intimidation),
            "medicine" => Ok(Skill::Medicine),
            "nature" => Ok(Skill::Nature),
            "occultism" => Ok(Skill::Occultism),
            "performance" => Ok(Skill::Performance),
            "religion" => Ok(Skill::Religion),
            "society" => Ok(Skill::Society),
            "stealth" => Ok(Skill::Stealth),
            "survival" => Ok(Skill::Survival),
            "thievery" => Ok(Skill::Thievery),
            _ => Err(mlua::Error::external(format!("Invalid Skill: {}", s)))
        }
    }
}