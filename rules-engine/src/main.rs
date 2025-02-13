use rules_engine::{compile_rule_item, pathfinder2e::compiler::compile_class::PathfinderClassCompiler};

pub fn main() {
    compile_rule_item::<PathfinderClassCompiler>("./pathfinder2e-def").expect("There was a problem running the code");
}