pub mod feature;
pub mod stats;

use std::{collections::HashMap, fmt::Display, path::Path};
use mlua::{Debug, Function, Lua, MetaMethod, UserData, UserDataFields, UserDataMethods, UserDataRefMut, UserDataRegistry, Value};
use crate::schema::ValueSchema;

#[derive(Clone)]
pub struct Character {
    schema : ValueSchema
}

impl UserData for Character {
    // fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
    //     fields.add_meta_field(MetaMethod::NewIndex, Function::wrap(|mut this : UserDataRefMut<Self>, key: String, value: Value| {
    //         let field = match value {
    //             Value::String(str) => CharacterField::Simple(CharacterFieldValue::String(str.to_string_lossy())),
    //             Value::Number(num) => CharacterField::Simple(CharacterFieldValue::Number(num)),
    //             Value::Function(func) => {
    //                 println!("{:?}", func.environment());
    //                 return Ok(())
    //             },
    //             _ => return Ok(())
    //         };
    //         this.fields.insert(key, field);
    //         Ok(())
    //     }));
    // }

    fn register(registry: &mut UserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }

    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {}

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        
    }
}

//==============================================================================================
//        CharacterField
//==============================================================================================

#[derive(Clone)]
pub enum CharacterField {
    Simple(CharacterFieldValue),
    Calculated(Vec<u8>, CharacterFieldValue, Vec<String>)
}

#[derive(Clone)]
pub enum CharacterFieldValue {
    String(String),
    Number(f64)
}

impl Display for CharacterFieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        
        match self {
            CharacterFieldValue::String(s) => s.fmt(f),
            CharacterFieldValue::Number(n) => n.fmt(f),
        }
    }
}