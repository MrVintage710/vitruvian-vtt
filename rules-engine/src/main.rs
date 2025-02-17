use rules_engine::{compile_rule_item, pathfinder2e::compiler::Pathfinder2eCompiler};

pub fn main() {
    compile_rule_item::<Pathfinder2eCompiler>("./pathfinder2e-def").expect("There was a problem running the code");
}