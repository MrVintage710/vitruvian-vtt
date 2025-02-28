//==============================================================================================
//        PakIndexMap
//==============================================================================================

use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{value::Value, PakPointer};

pub type PakIndices = HashMap<Value, Vec<PakPointer>>;

//==============================================================================================
//        PakIndex
//==============================================================================================

#[derive(PartialEq, Debug, Clone, PartialOrd, Deserialize, Serialize)]
pub struct PakIndex {
    pub key : String,
    pub value : Value
}

impl PakIndex {
    pub fn new(key : String, value : Value) -> Self {
        Self {
            key,
            value,
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::*;
    
}