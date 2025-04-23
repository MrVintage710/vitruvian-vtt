use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::Index};

use mlua::{Error, FromLua, Function, IntoLua, Lua, Result, Value};

//==============================================================================================
//        Stats
//==============================================================================================

#[derive(Default, Clone, Debug)]
pub struct StatBlock {
    values : HashMap<String, StatValue>,
    // In the future I may decide to add a cache for calculated stats
    // calculated : HashMap<String, (Vec<u8>, StatValue)>,
    // deps_map : HashMap<String, Vec<String>>
}

impl Index<&str> for StatBlock {
    type Output = StatValue;

    fn index(&self, index: &str) -> &Self::Output {
        let Some(value) = self.values.get(index) else {
            return &StatValue::Nil;
        };
        
        if let StatValue::Calc(bytes, _) = value {
            let lua = Lua::new();
            let value : StatValue = lua.load(bytes).eval().expect("There was a problem while calculating a stat value.");
            // return &value;
        }
        
        value
    }
}

impl IntoLua for StatBlock {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        todo!()
    }
}

impl FromLua for StatBlock {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        match value {
            Value::Table(t) => {
                let mut values = HashMap::new();
                for pair in t.pairs::<String, Value>() {
                    let (key, value) = pair?;
                    values.insert(key, StatValue::from_lua(value, lua)?);
                }
                Ok(StatBlock { values })
            },
            _ => Err(Error::FromLuaConversionError { from: value.type_name(), to: "StatBlock".to_string(), message: None })
        }
    }
}

//==============================================================================================
//        StatValue
//==============================================================================================

#[derive(Clone, Default)]
pub enum StatValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Group(HashMap<String, StatValue>),
    Calc(Vec<u8>, RefCell<Box<StatValue>>),
    #[default]
    Nil
}

impl IntoLua for StatValue {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        match self {
            StatValue::Integer(i) => i.into_lua(lua),
            StatValue::Float(n) => n.into_lua(lua),
            StatValue::String(s) => s.into_lua(lua),
            StatValue::Boolean(b) => b.into_lua(lua),
            StatValue::Group(b) => b.into_lua(lua),
            StatValue::Calc(v, _) => {
                let func : Function = lua.load(v).into_function()?;
                Ok(Value::Function(func))
            }
            StatValue::Nil => Ok(Value::Nil),
        }
    }
}

impl FromLua for StatValue {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        match value {
            Value::Number(n) => Ok(StatValue::Float(n)),
            Value::Integer(n) => Ok(StatValue::Integer(n)), 
            Value::String(s) => Ok(StatValue::String(s.to_string_lossy())),
            Value::Boolean(b) => Ok(StatValue::Boolean(b)),
            Value::Table(t) => {
                let stat_block = HashMap::<String, StatValue>::from_lua(Value::Table(t), lua)?;
                Ok(StatValue::Group(stat_block))
            },
            Value::Function(f) => {
                println!("{:?} | {:?}-{:?}", f.info().source, f.info().line_defined, f.info().last_line_defined);
                Ok(StatValue::Calc(f.dump(false), RefCell::new(Box::new(StatValue::Nil))))
            },
            Value::Nil => Ok(StatValue::Nil),
            _ => Err(Error::FromLuaConversionError { from: value.type_name(), to: "StatValue".to_string(), message: None })
        }
    }
}

impl Debug for StatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatValue::Integer(n) => write!(f, "{}", n),
            StatValue::Float(n) => write!(f, "{}", n),
            StatValue::String(s) => write!(f, "{}", s),
            StatValue::Boolean(b) => write!(f, "{}", b),
            StatValue::Group(b) => b.fmt(f),
            StatValue::Calc(_, _) => write!(f, "Calc"),
            StatValue::Nil => write!(f, "Nil"),
        }
    }
}

impl StatValue {
    
}
