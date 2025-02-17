use std::path::Path;
use error::VitruvianRulesEngineResult;
use mlua::{Lua, Table};

pub mod pathfinder2e;
pub mod common;
pub mod error;

#[derive(Default, Debug, Clone)]
pub struct ObjectIdentifier(Vec<String>);

impl ObjectIdentifier {    
    pub fn next(&self, id: String) -> ObjectIdentifier {
        let mut new = self.clone();
        new.0.push(id);
        new
    }
    
    pub fn peel(&self) -> ObjectIdentifier {
        let mut new = self.clone();
        new.0.pop();
        new
    }
    
    pub fn push(&mut self, id: String) {
        self.0.push(id);
    }
    
    pub fn id(&self) -> String {
        self.0.join("::")
    }
}

pub trait RulesModuleGenerator {
    fn generate_module(&self);
}

pub trait RulesItemCompiler {
    type ResultType;
    
    fn get_identifier() -> &'static str {
        "*"
    }
    
    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        Ok(Lua::new())
    }
    
    fn compile(lua : &Lua, src : &str, item_name : ObjectIdentifier) -> VitruvianRulesEngineResult<Self::ResultType>;
}

pub fn compile_rule_item<R : RulesItemCompiler>(path : impl AsRef<Path>) -> VitruvianRulesEngineResult<()> {
    let lua = R::prepare_lua()?;
    
    //Setting the path to allow importing other files
    let path = path.as_ref().to_str().unwrap().to_string();
    let package : Table = lua.globals().get("package")?;
    package.set("path", format!("{path}/?.lua;;"))?;
    
    compile_rule_objects_r::<R>(path, &lua, ObjectIdentifier::default())?;
    
    Ok(())
}

fn compile_rule_objects_r<R : RulesItemCompiler>(path : impl AsRef<Path>, lua : &Lua, id : ObjectIdentifier) -> VitruvianRulesEngineResult<()> {
    let dir = std::fs::read_dir(&path)?;
    
    for file in dir {
        let file = file?;
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let item_name = file_name.chars().take_while(|&c| c != '.').collect::<String>();
        let id = id.next(item_name);
        
        if path.is_dir() {
            compile_rule_objects_r::<R>(path, lua, id)?;
        } else {
            if R::get_identifier() == "*" || file_name.ends_with(&format!(".{}.lua", R::get_identifier())) {
                let source = std::fs::read_to_string(path)?;
                // let item_name = file_name.chars().map(|c| if c == '_' {return ' '} else { c } ).take_while(|&c| c != '.').collect::<String>();
                R::compile(&lua, &source, id)?;
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_compile() {
        
    }
}
