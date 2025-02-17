use mlua::{Function, Lua, Value};
use crate::{error::VitruvianRulesEngineResult, ObjectIdentifier, RulesItemCompiler};
use super::{action::Action, class::ClassMeta, feature::{Feature, FeatureMeta}, passive::Passive};

pub fn globals(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    
    //functions
    func_define_class(lua)?;
    func_define_feature(lua)?;
    func_feature_passive(lua)?;
    func_feature_action(lua)?;
    func_feature_choice(lua)?;
    func_feature_group(lua)?;
    
    Ok(())
}

pub fn func_define_class(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    let define_class = lua.create_function(|lua : &Lua, meta : ClassMeta| {
        // println!("CLASS DEFINED: {:?}", meta);
        lua.globals().set("__pathfinder2e_class_meta", meta)?;
        
        Ok(())
    })?;
    
    lua.globals().set("define_class", Value::Function(define_class))?;
    Ok(())
}

pub fn func_define_feature(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    let define_feature = lua.create_function(|lua : &Lua, feature : Feature| {
        // println!("FEATURE DEFINED: {:?}", feature);
        lua.globals().set("__pathfinder2e_feature_meta", feature)?;
        
        Ok(())
    })?;
    
    lua.globals().set("define_feature", Value::Function(define_feature))?;
    Ok(())
}

pub fn func_feature_action(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    let feature_action = lua.create_function(|_ : &Lua, (action, meta) : (Action, Option<FeatureMeta>) | {
        let feature = Feature::Action { meta: meta.unwrap_or_default(), action };
        Ok(feature)
    })?;
    
    lua.globals().set("feature_action", Value::Function(feature_action))?;
    Ok(())
}

pub fn func_feature_passive(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    let feature_passive = lua.create_function(|_lua : &Lua, (passive, meta) : (Passive, Option<FeatureMeta>) | {
        let feature = Feature::Passive { meta: meta.unwrap_or_default(), passive };
        Ok(feature)
    })?;
    
    lua.globals().set("feature_passive", Value::Function(feature_passive))?;
    Ok(())
}

pub fn func_feature_choice(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    let feature_choice = lua.create_function(|_lua : &Lua, (features, number_of_choices, meta) : (Vec<Feature>, Option<u8>, Option<FeatureMeta>) | {
        let feature = Feature::Choice { meta: meta.unwrap_or_default(), number_of_choices : number_of_choices.unwrap_or(1), features};
        Ok(feature)
    })?;
    
    lua.globals().set("feature_choice", Value::Function(feature_choice))?;
    Ok(())
}

pub fn func_feature_group(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    let feature_group = lua.create_function(|_lua : &Lua, (features, meta) : (Vec<Feature>, Option<FeatureMeta>) | {
        let feature = Feature::Group { meta: meta.unwrap_or_default(), features };
        Ok(feature)
    })?;
    
    lua.globals().set("feature_group", Value::Function(feature_group))?;
    Ok(())
}

pub struct Pathfinder2eCompiler;

impl RulesItemCompiler for Pathfinder2eCompiler {
    type ResultType = ();
    
    fn compile(lua : &Lua, src : &str, item_name: ObjectIdentifier) -> VitruvianRulesEngineResult<Self::ResultType> {
        lua.globals().set("id", item_name.0)?;
        
        let value : mlua::Value = lua.load(src).eval()?;
        
        Ok(())
    }

    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        let lua = Lua::new();
        globals(&lua)?;
        Ok(lua)
    }

    fn get_identifier() -> &'static str {
        "p2fe"
    }
}