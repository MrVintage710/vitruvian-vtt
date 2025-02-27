

//==============================================================================================
//        Pak Values
//==============================================================================================

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Deserialize, Serialize, PartialEq, Clone, PartialOrd, Debug, Display, Eq, Hash)]
pub enum Value {
    String(String),
    F64(u64),
    F32(u32),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    Boolean(bool),
}

impl Value {
    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(bits) => Some(f64::from_bits(*bits)),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::F32(bits) => Some(f32::from_bits(*bits)),
            _ => None,
        }
    }
    
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::U32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Value::U16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Value::U8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Value::I16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Value::I8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(value) => Some(*value),
            _ => None,
        }
    }
}

impl<'s> From<&'s str> for Value {
    fn from(value: &'s str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value.to_bits())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value.to_bits())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}
