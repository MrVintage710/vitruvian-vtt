pub mod pathfinder2e;
pub mod common;
pub mod error;
pub mod compile;
pub mod item;

pub trait RulesModuleGenerator {
    fn generate_module(&self);
}