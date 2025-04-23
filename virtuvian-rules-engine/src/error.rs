use thiserror::Error;

pub type VreResult<T> = Result<T, VreError>;
pub type SchemaResult<T> = Result<T, SchemaError>;

//==============================================================================================
//        Errors
//==============================================================================================

#[derive(Error, Debug)]
pub enum VreError {
    #[error("Lua Error: {0}")]
    LuaError(#[from] mlua::Error),
    
    #[error("Schema Check Failed: {0}")]
    SchemaError(#[from] SchemaError),
}

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Type Mismatch: Expected {expected}, got {actual} at {path}")]
    TypeMismatch {
        expected: String,
        actual: String,
        path: String,
    },
    
    #[error("Missing Key: {key} at {path}")]
    MissingKey {
        key: String,
        path: String,
    }
}