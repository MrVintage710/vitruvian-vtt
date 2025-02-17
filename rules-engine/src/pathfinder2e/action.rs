use std::ops::Range;

use mlua::{FromLua, IntoLua, Value};
use serde::{Deserialize, Serialize};

use super::traits::Trait;

#[derive(Deserialize, Serialize, Debug)]
pub struct Action {
    pub cost : ActionCost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements : Option<String>,
    pub rules_text : String,
    pub traits : Vec<Trait>,
}

impl Action {
    pub fn new(cost : ActionCost, rules_text : &str) -> Self {
        Action {
            cost,
            trigger: None,
            requirements: None,
            rules_text: rules_text.to_string(),
            traits: vec![]
        }
    }
    
    pub fn with_trigger(mut self, trigger : &str) -> Self {
        self.trigger = Some(trigger.to_string());
        self
    }
    
    pub fn with_requirements(mut self, requirements : &str) -> Self {
        self.requirements = Some(requirements.to_string());
        self
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

impl IntoLua for Action {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        let table = lua.create_table()?;
        table.set("cost", self.cost)?;
        table.set("trigger", self.trigger)?;
        table.set("requirements", self.requirements)?;
        table.set("rules_text", self.rules_text)?;
        table.set("traits", self.traits)?;
        Ok(Value::Table(table))
    }
}

impl FromLua for Action {
    fn from_lua(value: Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if let Value::Table(table) = value {
            let cost = table.get("cost")?;
            // let trigger = table.get("trigger")?;
            // let requirements = table.get("requirements")?;
            let rules_text = table.get("rulesText")?;
            let traits = table.get("traits")?;
            Ok(Action {
                cost,
                trigger : None,
                requirements : None,
                rules_text,
                traits,
            })
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().to_string(),
                message: Some("Must be a table to represent an action.".to_string()),
            })
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ActionCost {
    One,
    Two,
    Three,
    Reaction,
    Free,
    Range(Range<u8>)
}

impl IntoLua for ActionCost {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            ActionCost::One => Ok(Value::Number(1.into())),
            ActionCost::Two => Ok(Value::Number(2.into())),
            ActionCost::Three => Ok(Value::Number(3.into())),
            ActionCost::Reaction => lua.create_string("reaction").map(|s| Value::String(s)),
            ActionCost::Free => lua.create_string("free").map(|s| Value::String(s)),
            ActionCost::Range(range) => range.into_iter().collect::<Vec<u8>>().into_lua(lua),
        }
    }
}

impl FromLua for ActionCost {
    fn from_lua(value: Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let from_type = value.type_name();
        let to_type = std::any::type_name::<Self>();
        
        match value {
            Value::Integer(number) => from_number(number, from_type, to_type),
            Value::Number(number) => from_number(number.round() as i64, from_type, to_type),
            Value::String(string) => from_string(string.to_str()?.to_string(), from_type, to_type),
            _ => Err(mlua::Error::FromLuaConversionError { 
                from: from_type, 
                to: to_type.to_string(), 
                message: Some("The value must be either a table, string or number.".to_string())
            })
        }
    }
}

fn from_number(number : i64, from_type : &'static str, to_type : &'static str) -> mlua::Result<ActionCost> {
    match number {
        1 => Ok(ActionCost::One),
        2 => Ok(ActionCost::Two),
        3 => Ok(ActionCost::Three),
        _ => Err(mlua::Error::FromLuaConversionError { 
            from: from_type, 
            to: to_type.to_string(), 
            message: Some("An ActionCost can be no more than 3 actions.".to_string())
        })
    }
}

fn from_string(string : String, from_type : &'static str, to_type : &'static str) -> mlua::Result<ActionCost> {
    match string.as_str() {
        "1-2" => Ok(ActionCost::Range(1..2)),
        "1-3" => Ok(ActionCost::Range(1..3)),
        "reaction" => Ok(ActionCost::Reaction),
        "free" => Ok(ActionCost::Free),
        _ => Err(mlua::Error::FromLuaConversionError { 
            from: from_type, 
            to: to_type.to_string(), 
            message: Some(format!("The string must be either 'reaction' or 'free'. Recieved: {}", string))
        })
    }
}
