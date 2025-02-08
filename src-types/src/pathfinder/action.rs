use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::traits::Trait;

#[derive(Deserialize, Serialize)]
pub struct Action {
    pub name : String,
    pub cost : ActionCost,
    pub trigger : Option<String>,
    pub requirements : Option<String>,
    pub rules_text : String,
    pub traits : Vec<Trait>,
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