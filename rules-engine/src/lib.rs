use std::path::Path;
use error::VitruvianRulesEngineResult;
use mlua::{Lua, Table};

pub mod pathfinder2e;
pub mod common;
pub mod error;
pub mod compile;
pub mod item;
pub mod pak;

pub trait RulesModuleGenerator {
    fn generate_module(&self);
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_compile() {
        
    }
}
