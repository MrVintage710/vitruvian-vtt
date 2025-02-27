use std::{fs::File, io::BufReader};

use meta::PakMeta;
use serde::{Deserialize, Serialize};

use crate::error::VitruvianRulesEngineResult;

pub mod meta;
pub mod item;
pub mod index;
pub mod value;

//==============================================================================================
//        Pak Create Function
//==============================================================================================

pub fn create_pak(path : &str, ) -> VitruvianRulesEngineResult<()> {
    Ok(())
}

//==============================================================================================
//        Pak Metadata
//==============================================================================================

pub struct PakFile {
    meta : PakMeta,
    stream : BufReader<File>
}

impl PakFile {
    // pub fn new(path: &str) -> Result<Self, std::io::Error> {
    //     let file = File::open(path)?;
    //     let stream = BufReader::new(file);
        
    //     // let meta = PakMeta::from_reader(&stream)?;

    //     Ok(Self { meta, stream })
    // }
}

//==============================================================================================
//        PakPointer
//==============================================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PakPointer {
    offset : u64,
    size : u64,
}

impl PakPointer {
    pub fn new(offset : u64, size : u64) -> Self {
        Self { offset, size }
    }
}
