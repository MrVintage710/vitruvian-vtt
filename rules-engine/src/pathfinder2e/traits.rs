use std::str::FromStr;

use mlua::{FromLua, IntoLua, Lua, Result as LuaResult, Value};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
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

impl IntoLua for Trait {
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        let name = self.to_string().to_lowercase();
        return Ok(Value::String(lua.create_string(name)?));
    }
}

impl FromLua for Trait {
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        if !value.is_string() { 
            return Err(mlua::Error::FromLuaConversionError { 
                from: value.type_name(), 
                to: std::any::type_name::<Self>().to_string(), 
                message: Some("Must be a string to convert.".to_string()) 
            });
        }
        let name = value.as_str().unwrap().to_lowercase();
        return Ok(Trait::from_str(&name).map_err(mlua::Error::external)?);
    }
}