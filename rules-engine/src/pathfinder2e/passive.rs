use serde::{Deserialize, Serialize};
use super::traits::Trait;


#[derive(Deserialize, Serialize)]
pub struct Passive {
    pub rules_text : String,
    pub traits : Vec<Trait>,
}

impl Passive {
    pub fn new(rules_text: &str) -> Self {
        Passive {
            rules_text: rules_text.to_string(),
            traits : vec![],
        }
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