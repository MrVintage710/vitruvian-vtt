use std::path::Path;
use error::VitruvianRulesEngineResult;
use mlua::{Lua, Table};

pub mod pathfinder2e;
pub mod common;
pub mod error;

pub trait RulesModuleGenerator {
    fn generate_module(&self);
}

pub trait RulesItemCompiler {
    type ResultType;
    
    fn get_identifier() -> &'static str;
    
    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        Ok(Lua::new())
    }
    
    fn compile(lua : &Lua, src : &str, item_name : &str) -> VitruvianRulesEngineResult<Self::ResultType>;
}

pub fn compile_rule_item<R : RulesItemCompiler>(path : impl AsRef<Path>) -> VitruvianRulesEngineResult<Vec<R::ResultType>> {
    let dir = std::fs::read_dir(&path)?;
    let path = path.as_ref().to_str().unwrap().to_string();
    
    let mut item = vec![];
    let lua = R::prepare_lua()?;
    
    //Setting the path to allow importing other files
    let package : Table = lua.globals().get("package")?;
    package.set("path", format!("{path}/?.lua;{path}/../?.lua"))?;
    
    for file in dir {
        let file = file?;
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        
        if path.is_dir() {
            item.append(&mut compile_rule_item::<R>(path)?);
        } else {
            if file_name.ends_with(&format!(".{}.lua", R::get_identifier())) {
                let source = std::fs::read_to_string(path)?;
                let item_name = file_name.chars().map(|c| if c == '_' {return ' '} else { c } ).take_while(|&c| c != '.').collect::<String>();
                R::compile(&lua, &source, &item_name)?;
                // let class =compile_class(std::fs::read_to_string(path)?)?;
                // classes.push(class);
            }
        }
    }
    
    Ok(item)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_compile() {
        
    }
}
