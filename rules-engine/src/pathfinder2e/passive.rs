use mlua::{FromLua, IntoLua, Value};
use serde::{Deserialize, Serialize};
use super::traits::Trait;


#[derive(Deserialize, Serialize, Debug)]
pub struct Passive {
    pub rules_text : String,
    pub traits : Vec<Trait>,
}

impl Passive {
    pub fn new(rules_text: &str) -> Self {
        Passive {
            rules_text: rules_text.to_string(),
            traits : vec![],
        }
    }
    
    pub fn with_trait(mut self, t: Trait) -> Self {
        self.traits.push(t);
        self
    }
    
    pub fn with_traits(mut self, ts: Vec<Trait>) -> Self {
        self.traits.extend(ts);
        self
    }
}

impl IntoLua for Passive {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;
        table.set("rulesText", self.rules_text)?;
        table.set("traits", self.traits)?;
        Ok(Value::Table(table))
    }
}

impl FromLua for Passive {
    fn from_lua(value: Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Passive>().to_string(),
                message: None,
            });
        }
        let table = value.as_table().unwrap();
        let rules_text = table.get("rulesText")?;
        let traits = table.get("traits")?;
        Ok(Passive {
            rules_text,
            traits,
        })
    }
}
