//==============================================================================================
//        PakIndexMap
//==============================================================================================

use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{value::Value, PakPointer};

#[derive(Serialize, Deserialize)]
pub struct PakIndices {
    indices : HashMap<Value, Vec<PakPointer>>
}

impl PakIndices {
    
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
        }
    }
}

//==============================================================================================
//        PakIndex
//==============================================================================================

#[derive(PartialEq, Debug, Clone, PartialOrd, Deserialize, Serialize)]
pub struct PakIndex {
    key : String,
    value : Value
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