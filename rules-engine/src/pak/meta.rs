use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PakMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub indices_size: usize,
    pub objects_size: usize,
}