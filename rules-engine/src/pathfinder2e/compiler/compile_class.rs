use mlua::{Lua, String, Table, Value};
use crate::{error::VitruvianRulesEngineResult, pathfinder2e::class::ClassMeta, RulesItemCompiler};

pub struct PathfinderClassCompiler;

impl RulesItemCompiler for PathfinderClassCompiler {
    type ResultType = ();
    
    fn get_identifier() -> &'static str {
        "class"
    }
    
    fn compile(lua : &Lua, src : &str, item_name: &str) -> VitruvianRulesEngineResult<Self::ResultType> {
        
        let class_name_val = item_name.to_string();
        let class_name = lua.create_function(move |lua : &Lua, () : ()| {
            return Ok(lua.create_string(class_name_val.clone()));
        })?;
        
        lua.globals().set("class_name", Value::Function(class_name))?;
        
        let value : mlua::Value = lua.load(src).eval()?;
        
        println!("Return value: {:?}", value);
        
        Ok(())
    }

    fn prepare_lua() -> VitruvianRulesEngineResult<Lua> {
        let lua = Lua::new();
        
        let define_class = lua.create_function(|lua : &Lua, meta : ClassMeta| {
            println!("CLASS DEFINED: {:?}", meta);
            lua.globals().set("__pathfinder2e_class_meta", meta)?;
            
            Ok(())
        })?;
        
        lua.globals().set("define_class", Value::Function(define_class))?;
        
        // let define_class = lua.create_function(|lua : &Lua, () : ()|)?;
        
        Ok(lua)
    }
}

// pub fn compile_classes(path : impl AsRef<Path>) -> VitruvianRulesEngineResult<Vec<Class>> {
//     let dir = std::fs::read_dir(path)?;
    
//     let mut classes = vec![];
    
//     for file in dir {
//         let file = file?;
//         let path = file.path();
//         println!("{:?}", path);
//         let file_name = path.file_name().unwrap().to_str().unwrap();
        
//         if path.is_dir() {
//             classes.append(&mut compile_classes(path)?);
//         } else {
//             if file_name.ends_with(".class.lua") {
//                 compile_class(std::fs::read_to_string(path)?)?;
//                 // let class =compile_class(std::fs::read_to_string(path)?)?;
//                 // classes.push(class);
//             }
//         }
//     }
    
//     Ok(classes)
// }

pub fn compile_class(content : String) -> VitruvianRulesEngineResult<()> {
    Ok(())
}