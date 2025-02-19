use mlua::{Function, Lua, Value};
use crate::{compile::RulesItemCompiler, error::VitruvianRulesEngineResult, item::RulesItemIdentifier};
use super::{action::Action, class::{Class, ClassMeta}, feature::{Feature, FeatureMeta}, passive::Passive};

//==============================================================================================
//        Lua Globals
//==============================================================================================

pub fn globals(lua : &Lua) -> VitruvianRulesEngineResult<()> {
    
    //functions
    func_feature_passive(lua)?;
    func_feature_action(lua)?;
    func_feature_choice(lua)?;
    func_feature_group(lua)?;
    
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

//==============================================================================================
//        Class Compiler
//==============================================================================================

pub struct Pathfinder2eClassCompiler;

impl RulesItemCompiler for Pathfinder2eClassCompiler {
    type ResultType = ();
    
    fn parse(lua : &Lua, src : &str, item_name: RulesItemIdentifier) -> VitruvianRulesEngineResult<Self::ResultType> {
        lua.globals().set("id", item_name.id())?;
        
        let value : Option<ClassMeta> = lua.load(src).eval()?;
        
        Ok(())
    }

    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        let lua = Lua::new();
        globals(&lua)?;
        Ok(lua)
    }

    fn get_identifier() -> &'static str {
        "class"
    }

    fn compile(results : Vec<Self::ResultType>) -> VitruvianRulesEngineResult<()> {
        todo!()
    }
}

//==============================================================================================
//        Feature Compiler
//==============================================================================================

pub struct Pathfinder2eFeatureCompiler;

impl RulesItemCompiler for Pathfinder2eFeatureCompiler {
    type ResultType = ();
    
    fn parse(lua : &Lua, src : &str, item_name: RulesItemIdentifier) -> VitruvianRulesEngineResult<Self::ResultType> {
        lua.globals().set("id", item_name.id())?;
        
        let value : Option<Feature> = lua.load(src).eval()?;
        let on_activate : Function = lua.globals().get("OnActivate")?;
        lua.load(on_activate.dump(false)).exec()?;
        
        Ok(())
    }

    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        let lua = Lua::new();
        globals(&lua)?;
        Ok(lua)
    }

    fn get_identifier() -> &'static str {
        "feature"
    }

    fn compile(results : Vec<Self::ResultType>) -> VitruvianRulesEngineResult<()> {
        todo!()
    }
}
