use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::traits::Trait;

#[derive(Deserialize, Serialize)]
pub struct Action {
    pub cost : ActionCost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements : Option<String>,
    pub rules_text : String,
    pub traits : Vec<Trait>,
}

impl Action {
    
    pub fn new(cost : ActionCost, rules_text : &str) -> Self {
        Action {
            cost,
            trigger: None,
            requirements: None,
            rules_text: rules_text.to_string(),
            traits: vec![]
        }
    }
    
    pub fn with_trigger(mut self, trigger : &str) -> Self {
        self.trigger = Some(trigger.to_string());
        self
    }
    
    pub fn with_requirements(mut self, requirements : &str) -> Self {
        self.requirements = Some(requirements.to_string());
        self
    }
    
    pub fn with_trait(mut self, t: Trait) -> Self {
        self.traits.push(t);
        self
    }
    
    pub fn with_traits(mut self, ts: Vec<Trait>) -> Self {
        self.traits.extend(ts);
        self
    }
}

#[derive(Deserialize, Serialize)]
pub enum ActionCost {
    One,
    Two,
    Three,
    Reaction,
    Free,
    Range(Range<u8>)
}