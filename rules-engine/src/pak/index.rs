//==============================================================================================
//        PakIndexMap
//==============================================================================================

use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{value::PakValue, PakPointer};

pub type PakIndices = HashMap<PakValue, Vec<PakPointer>>;



//==============================================================================================
//        PakIndex
//==============================================================================================

#[derive(PartialEq, Debug, Clone, PartialOrd, Deserialize, Serialize)]
pub struct PakIndex {
    pub key : String,
    pub value : PakValue
}

impl PakIndex {
    pub fn new(key : &str, value : impl Into<PakValue>) -> Self {
        Self {
            key: key.to_string(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::*;
    
}