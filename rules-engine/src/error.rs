use std::io;
use thiserror::Error;

pub type VitruvianRulesEngineResult<T> = Result<T, VitruvianRulesEngineError>;

#[derive(Error, Debug)]
pub enum VitruvianRulesEngineError {
    #[error("There was an error parsing the rules folder: {0}")]
    IoError(#[from] io::Error),
    #[error("There was an error running the lua file: {0}")]
    LuaError(#[from] mlua::Error),
    #[error("There was an error while converting a string to an enum: {0}")]
    EnumCoerceError(#[from] strum::ParseError),
}