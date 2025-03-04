use mlua::{FromLua, IntoLua};
use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, IntoDiscriminant, IntoStaticStr};

use crate::pak::value::PakValue;


#[derive(Deserialize, Serialize, Debug, EnumDiscriminants, IntoStaticStr, Clone)]
#[strum_discriminants(name(SourceBook))]
#[strum_discriminants(derive(IntoStaticStr))]
pub enum SourceRef {
    PlayerCore(u16),
    PlayerCore2(u16),
    WarOfImmortals(u16),
}

impl SourceRef {
    pub fn page(&self) -> u16 {
        match self {
            SourceRef::PlayerCore(v) => *v,
            SourceRef::PlayerCore2(v) => *v,
            SourceRef::WarOfImmortals(v) => *v,
        }
    }
}

impl Into<PakValue> for SourceRef {
    fn into(self) -> PakValue {
        let s : &'static str = self.into();
        PakValue::String(s.to_string())
    }
}

impl IntoLua for SourceRef {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        let key : &'static str = self.discriminant().into();
        table.set(key, self.page())?;
        Ok(mlua::Value::Table(table))
    }
}

impl FromLua for SourceRef {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() { return Err(mlua::Error::external("Invalid source ref value")); }
        let table = value.as_table().unwrap();
        let (key, value) : (String, u16) = table.pairs().next().unwrap()?;
        match key.as_str() {
            "PlayerCore" => Ok(SourceRef::PlayerCore(value)),
            "PlayerCore2" => Ok(SourceRef::PlayerCore2(value)),
            "WarOfImmortals" => Ok(SourceRef::WarOfImmortals(value)),
            _ => Err(mlua::Error::external("Invalid source ref value"))
        }
    }
}