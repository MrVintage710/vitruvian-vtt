use mlua::{FromLua, IntoLua, Value};
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::{common::DiceAmount, pak::value::PakValue as PakValue};

pub mod class;
pub mod action;
pub mod passive;
pub mod skill;
pub mod traits;
pub mod source;
pub mod compiler;
pub mod feature;

//=========================================================================================================================
//         Common Types
//=========================================================================================================================

// Attribute
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Attribute {
    pub fn from_string(s : &str) -> Option<Attribute> {
        match s {
            "str" => Some(Attribute::Strength),
            "dex" => Some(Attribute::Dexterity),
            "con" => Some(Attribute::Constitution),
            "int" => Some(Attribute::Intelligence),
            "wis" => Some(Attribute::Wisdom),
            "cha" => Some(Attribute::Charisma),
            _ => None
        }
    }
}

impl Into<PakValue> for Attribute {
    fn into(self) -> PakValue {
        match self {
            Attribute::Strength => "str".into(),
            Attribute::Dexterity => "dex".into(),
            Attribute::Constitution => "con".into(),
            Attribute::Intelligence => "int".into(),
            Attribute::Wisdom => "wis".into(),
            Attribute::Charisma => "cha".into(),
        }
    }
}

impl IntoLua for Attribute {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        match self {
            Attribute::Strength => Ok(Value::String(lua.create_string("str")?)),
            Attribute::Dexterity => Ok(Value::String(lua.create_string("dex")?)),
            Attribute::Constitution => Ok(Value::String(lua.create_string("con")?)),
            Attribute::Intelligence => Ok(Value::String(lua.create_string("int")?)),
            Attribute::Wisdom => Ok(Value::String(lua.create_string("wis")?)),
            Attribute::Charisma => Ok(Value::String(lua.create_string("cha")?)),
        }
    }
}

impl FromLua for Attribute {
    fn from_lua(value: Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_string() {return Err(mlua::Error::external("Invalid attribute value"))}
        let value = value.as_string().unwrap();
        if let Some(attribute) = Attribute::from_string(&value.to_str()?) {
            Ok(attribute)
        } else {
            Err(mlua::Error::external("Invalid attribute value"))
        }
    }
}

// Attribue Boost
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Display)]
pub enum AttributeBoost {
    Single(Attribute),
    Double(Attribute, Attribute),
    Free
}

impl Into<PakValue> for AttributeBoost {
    fn into(self) -> PakValue {
        match self {
            AttributeBoost::Single(attribute) => attribute.into(),
            AttributeBoost::Double(attribute, attribute1) => (attribute, attribute1).into(),
            AttributeBoost::Free => "free".into(),
        }
    }
}

impl IntoLua for AttributeBoost {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> { 
        match self {
            AttributeBoost::Single(a) => {
                Ok(a.into_lua(lua)?)
            },
            AttributeBoost::Double(a1, a2) => {
                let table = lua.create_table()?;
                table.set(1, a1)?;
                table.set(2, a2)?;
                Ok(Value::Table(table))
            },
            AttributeBoost::Free => Ok(Value::String(lua.create_string("free")?))
        }
    }
}

impl FromLua for AttributeBoost {
    fn from_lua(value: Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => {
                let s = s.to_str()?;
                if s == "free" { return Ok(AttributeBoost::Free); }
                if let Some(attribute) = Attribute::from_string(&s) {
                    Ok(AttributeBoost::Single(attribute))
                } else {
                    Err(mlua::Error::external("Invalid attribute boost value"))
                }
            },
            Value::Table(t) => {
                let a1 : Attribute = t.get(1)?;
                let a2 : Attribute = t.get(2)?;
                Ok(AttributeBoost::Double(a1, a2))
            },
            _ => Err(mlua::Error::external("Invalid attribute boost value"))
        }
    }
}

// Proficiency
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
    Mythic
}

impl IntoLua for Proficiency {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        match self {
            Proficiency::Untrained => Ok(Value::String(lua.create_string("untrained")?)),
            Proficiency::Trained => Ok(Value::String(lua.create_string("trained")?)),
            Proficiency::Expert => Ok(Value::String(lua.create_string("expert")?)),
            Proficiency::Master => Ok(Value::String(lua.create_string("master")?)),
            Proficiency::Legendary => Ok(Value::String(lua.create_string("legendary")?)),
            Proficiency::Mythic => Ok(Value::String(lua.create_string("mythic")?)),
        }
    }
}

impl FromLua for Proficiency {
    fn from_lua(value: Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => {
                match s.to_str()?.as_ref() {
                    "untrained" => Ok(Proficiency::Untrained),
                    "trained" => Ok(Proficiency::Trained),
                    "expert" => Ok(Proficiency::Expert),
                    "master" => Ok(Proficiency::Master),
                    "legendary" => Ok(Proficiency::Legendary),
                    "mythic" => Ok(Proficiency::Mythic),
                    _ => Err(mlua::Error::external("Invalid proficiency value"))
                }
            },
            Value::Nil => { Ok(Proficiency::Untrained) },
            _ => { Err(mlua::Error::external("Invalid proficiency value")) }
        }
    }
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

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum DragonKind {
    Adamantine,
    Conspirator,
    Empyreal,
    Fortune,
    Horned,
    Mirage,
    Omen
}