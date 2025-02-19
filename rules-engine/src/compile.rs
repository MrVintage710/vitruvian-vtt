use std::{fmt::format, path::{Path, PathBuf}};
use rusqlite::{Connection, Result};
use mlua::{Lua, Table};
use sea_query::{Query};

use crate::{error::VitruvianRulesEngineResult, item::RulesItemIdentifier};


//==============================================================================================
//        RulesItemCompiler
//==============================================================================================

pub trait RulesItemCompiler {
    type ResultType;
    
    fn get_identifier() -> &'static str {
        "*"
    }
    
    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        Ok(Lua::new())
    }
    
    fn parse(lua : &Lua, src : &str, item_name : RulesItemIdentifier) -> VitruvianRulesEngineResult<Self::ResultType>;
    
    fn compile(results : Vec<Self::ResultType>) -> VitruvianRulesEngineResult<()>;
}

pub fn compile_rule_item<R : RulesItemCompiler>(path : impl AsRef<Path>) -> VitruvianRulesEngineResult<()> {
    let lua = R::prepare_lua()?;
    
    let path_name = path.as_ref().file_name().unwrap().to_str().unwrap();
    let mut p = PathBuf::from(path.as_ref());
    p.push(format!("{}.vmod", path_name));
    let connection = Connection::open(&p)?;
    
    
    
    //Setting the path to allow importing other files
    let path = path.as_ref().to_str().unwrap().to_string();
    let package : Table = lua.globals().get("package")?;
    package.set("path", format!("{path}/?.lua;;"))?;
    
    compile_rule_objects_r::<R>(path, &lua, &connection, RulesItemIdentifier::default())?;
    
    Ok(())
}

fn compile_rule_objects_r<R : RulesItemCompiler>(path : impl AsRef<Path>, lua : &Lua, connection : &Connection, id : RulesItemIdentifier) -> VitruvianRulesEngineResult<()> {
    let dir = std::fs::read_dir(&path)?;
    
    
    for file in dir {
        let file = file?;
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let item_name = file_name.chars().take_while(|&c| c != '.').collect::<String>();
        let id = id.next(item_name);
        
        if path.is_dir() {
            compile_rule_objects_r::<R>(path, lua, connection, id)?;
        } else {
            if R::get_identifier() == "*" || file_name.ends_with(&format!(".{}.lua", R::get_identifier())) {
                let source = std::fs::read_to_string(path)?;
                // let item_name = file_name.chars().map(|c| if c == '_' {return ' '} else { c } ).take_while(|&c| c != '.').collect::<String>();
                R::parse(&lua, &source, id)?;
            }
        }
    }
    
    Ok(())
}