use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum SourceRef {
    PlayerCore(u16),
    PlayerCore2(u16),
    WarOfImmortals(u16),
}