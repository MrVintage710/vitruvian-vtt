use rules_engine::{compile::compile_rule_item, pathfinder2e::compiler::{Pathfinder2eClassCompiler, Pathfinder2eFeatureCompiler}};

pub fn main() {
    compile_rule_item::<Pathfinder2eClassCompiler>("./pf2e").expect("There was a problem running the code");
    compile_rule_item::<Pathfinder2eFeatureCompiler>("./pf2e").expect("There was a problem running the code");
}