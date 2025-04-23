use mlua::{Function, Lua, ObjectLike, Table, Value};

use crate::{character::{feature::Feature, stats::StatBlock}, schema::ValueSchema};

//==============================================================================================
//        Feature Tests
//==============================================================================================

#[test]
fn test_create_feature() {
    
    let lua = Lua::new();
    
    lua.globals().set("feature", Function::wrap_raw(Feature::new)).unwrap();
    
    let value : String = lua.load(r#"
        local feature = feature("Test");
        return feature.name;
    "#).eval().unwrap();
    
    assert_eq!("Test", &value);
}

#[test]
fn character_stats_loading() {
    let lua = Lua::new();
    let character_stats_source = std::fs::read_to_string("pf2e-def/main.schema.lua").unwrap();
    let character_stats : StatBlock = lua.load(character_stats_source).eval().unwrap();
    
    println!("{:?}", character_stats)
}

#[test]
fn lua_schema() {
    let lua = Lua::new();
    let schema_source = std::fs::read_to_string("pf2e-def/main.schema.lua").unwrap();
    let value : mlua::Value = lua.load(schema_source).eval().unwrap();
    let value2 = Value::Nil;
    let schema = ValueSchema::derive(&value).unwrap();
    
    assert!(schema.check(&value));
    assert!(!schema.check(&value2));
    
    println!("{:?}", schema)
}