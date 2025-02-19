use std::io;
use thiserror::Error;

pub type VitruvianRulesEngineResult<T> = Result<T, VitruvianRulesEngineError>;

#[derive(Error, Debug)]
pub enum VitruvianRulesEngineError {
    #[error("Was unable to update rules item: {0}")]
    UpdateRuleItemError(String),
    #[error("Was unable to insert rules item: {0}")]
    InsertRuleItemError(String),
    
    #[error("There was an error parsing the rules folder: {0}")]
    IoError(#[from] io::Error),
    #[error("There was an error running the lua file: {0}")]
    LuaError(#[from] mlua::Error),
    #[error("There was an error while converting a string to an enum: {0}")]
    EnumCoerceError(#[from] strum::ParseError),
    #[error("There was an error while saving data to a module: {0}")]
    ModuleError(#[from] rusqlite::Error),
    #[error("There was an error while making a query: {0}")]
    QueryError(#[from] sea_query::error::Error),
}