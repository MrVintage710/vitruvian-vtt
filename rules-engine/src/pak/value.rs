

//==============================================================================================
//        Pak Values
//==============================================================================================

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Deserialize, Serialize, Clone, PartialOrd, Debug, Display, Eq, Hash, Default, Ord)]
pub enum PakValue {
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
    Group(Vec<PakValue>),
    #[default]
    Void
}

impl PartialEq for PakValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PakValue::String(a), PakValue::String(b)) => a == b,
            (PakValue::F64(a), PakValue::F64(b)) => a == b,
            (PakValue::F32(a), PakValue::F32(b)) => a == b,
            (PakValue::U64(a), PakValue::U64(b)) => a == b,
            (PakValue::U32(a), PakValue::U32(b)) => a == b,
            (PakValue::U16(a), PakValue::U16(b)) => a == b,
            (PakValue::U8(a), PakValue::U8(b)) => a == b,
            (PakValue::I64(a), PakValue::I64(b)) => a == b,
            (PakValue::I32(a), PakValue::I32(b)) => a == b,
            (PakValue::I16(a), PakValue::I16(b)) => a == b,
            (PakValue::I8(a), PakValue::I8(b)) => a == b,
            (PakValue::F64(a), PakValue::I64(b)) => f64::from_bits(*a) == (*b as f64),
            (PakValue::F32(a), PakValue::I32(b)) => f32::from_bits(*a) == (*b as f32),
            (PakValue::F64(a), PakValue::I32(b)) => f64::from_bits(*a) == (*b as f64),
            (PakValue::F32(a), PakValue::I16(b)) => f32::from_bits(*a) == (*b as f32),
            (PakValue::F64(a), PakValue::I16(b)) => f64::from_bits(*a) == (*b as f64),
            (PakValue::F32(a), PakValue::I8(b)) => f32::from_bits(*a) == (*b as f32),
            (PakValue::F64(a), PakValue::I8(b)) => f64::from_bits(*a) == (*b as f64),
            (PakValue::F32(a), PakValue::U32(b)) => f32::from_bits(*a) == (*b as f32),
            (PakValue::F64(a), PakValue::U32(b)) => f64::from_bits(*a) == (*b as f64),
            (PakValue::F32(a), PakValue::U16(b)) => f32::from_bits(*a) == (*b as f32),
            (PakValue::F64(a), PakValue::U16(b)) => f64::from_bits(*a) == (*b as f64),
            (PakValue::F32(a), PakValue::U8(b)) => f32::from_bits(*a) == (*b as f32),
            (PakValue::F64(a), PakValue::U8(b)) => f64::from_bits(*a) == (*b as f64),
            
            (PakValue::Boolean(a), PakValue::Boolean(b)) => a == b,
            (PakValue::Group(a), PakValue::Group(b)) => a == b,
            (PakValue::Void, PakValue::Void) => true,
            _ => false,
        }
    }
}

impl PakValue {
    pub fn as_string(&self) -> Option<String> {
        match self {
            PakValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PakValue::F64(bits) => Some(f64::from_bits(*bits)),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            PakValue::F32(bits) => Some(f32::from_bits(*bits)),
            _ => None,
        }
    }
    
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            PakValue::U64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            PakValue::U32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            PakValue::U16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            PakValue::U8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PakValue::I64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            PakValue::I32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            PakValue::I16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            PakValue::I8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PakValue::Boolean(value) => Some(*value),
            _ => None,
        }
    }
    
    pub fn f32(value : f32) -> Self {
        PakValue::F32(value.to_bits())
    }
    
    pub fn f64(value : f64) -> Self {
        PakValue::F64(value.to_bits())
    }
    
    pub fn i8(value : i8) -> Self {
        PakValue::I8(value)
    }
    
    pub fn i16(value : i16) -> Self {
        PakValue::I16(value)
    }
    
    pub fn i32(value : i32) -> Self {
        PakValue::I32(value)
    }
    
    pub fn i64(value : i64) -> Self {
        PakValue::I64(value)
    }

    pub fn u8(value : u8) -> Self {
        PakValue::U8(value)
    }
    
    pub fn u16(value : u16) -> Self {
        PakValue::U16(value)
    }
    
    pub fn u32(value : u32) -> Self {
        PakValue::U32(value)
    }
    
    pub fn u64(value : u64) -> Self {
        PakValue::U64(value)
    }
}

//==============================================================================================
//        Easy of use Traits
//==============================================================================================

impl<'s> From<&'s str> for PakValue {
    fn from(value: &'s str) -> Self {
        PakValue::String(value.to_string())
    }
}

impl From<String> for PakValue {
    fn from(value: String) -> Self {
        PakValue::String(value)
    }
}

impl From<f64> for PakValue {
    fn from(value: f64) -> Self {
        PakValue::F64(value.to_bits())
    }
}

impl From<f32> for PakValue {
    fn from(value: f32) -> Self {
        PakValue::F32(value.to_bits())
    }
}

impl From<i64> for PakValue {
    fn from(value: i64) -> Self {
        PakValue::I64(value)
    }
}

impl From<i32> for PakValue {
    fn from(value: i32) -> Self {
        PakValue::I32(value)
    }
}

impl From<i16> for PakValue {
    fn from(value: i16) -> Self {
        PakValue::I16(value)
    }
}

impl From<i8> for PakValue {
    fn from(value: i8) -> Self {
        PakValue::I8(value)
    }
}

impl From<u64> for PakValue {
    fn from(value: u64) -> Self {
        PakValue::U64(value)
    }
}

impl From<u32> for PakValue {
    fn from(value: u32) -> Self {
        PakValue::U32(value)
    }
}

impl From<u16> for PakValue {
    fn from(value: u16) -> Self {
        PakValue::U16(value)
    }
}

impl From<u8> for PakValue {
    fn from(value: u8) -> Self {
        PakValue::U8(value)
    }
}

impl From<bool> for PakValue {
    fn from(value: bool) -> Self {
        PakValue::Boolean(value)
    }
}

impl <T> From<Option<T>> for PakValue where T : Into<PakValue> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => PakValue::Void,
        }
    }
}

impl <T1, T2> From<(T1, T2)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue> {
    fn from(value: (T1, T2)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into()])
    }
}

impl <T1, T2, T3> From<(T1, T2, T3)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue> {
    fn from(value: (T1, T2, T3)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into()])
    }
}

impl <T1, T2, T3, T4> From<(T1, T2, T3, T4)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into()])
    }
}

impl <T1, T2, T3, T4, T5> From<(T1, T2, T3, T4, T5)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6> From<(T1, T2, T3, T4, T5, T6)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7> From<(T1, T2, T3, T4, T5, T6, T7)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7, T8> From<(T1, T2, T3, T4, T5, T6, T7, T8)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue>, T8 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into(), value.7.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7, T8, T9> From<(T1, T2, T3, T4, T5, T6, T7, T8, T9)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue>, T8 : Into<PakValue>, T9 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into(), value.7.into(), value.8.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> From<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue>, T8 : Into<PakValue>, T9 : Into<PakValue>, T10 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into(), value.7.into(), value.8.into(), value.9.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> From<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue>, T8 : Into<PakValue>, T9 : Into<PakValue>, T10 : Into<PakValue>, T11 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into(), value.7.into(), value.8.into(), value.9.into(), value.10.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> From<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue>, T8 : Into<PakValue>, T9 : Into<PakValue>, T10 : Into<PakValue>, T11 : Into<PakValue>, T12 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into(), value.7.into(), value.8.into(), value.9.into(), value.10.into(), value.11.into()])
    }
}

impl <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> From<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for PakValue where T1 : Into<PakValue>, T2 : Into<PakValue>, T3 : Into<PakValue>, T4 : Into<PakValue>, T5 : Into<PakValue>, T6 : Into<PakValue>, T7 : Into<PakValue>, T8 : Into<PakValue>, T9 : Into<PakValue>, T10 : Into<PakValue>, T11 : Into<PakValue>, T12 : Into<PakValue>, T13 : Into<PakValue> {
    fn from(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self {
        PakValue::Group(vec![value.0.into(), value.1.into(), value.2.into(), value.3.into(), value.4.into(), value.5.into(), value.6.into(), value.7.into(), value.8.into(), value.9.into(), value.10.into(), value.11.into(), value.12.into()])
    }
}

//==============================================================================================
//        Tests
//==============================================================================================

#[cfg(test)]
mod tests {
    use crate::pak::value::PakValue;

    
    #[test]
    fn pak_value_compare() {
        assert!(PakValue::f32(2.0) == PakValue::f32(2.0));
        assert!(PakValue::i32(40) > PakValue::u32(50));
    }
}