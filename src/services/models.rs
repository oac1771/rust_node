use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub meta_data: String,
    pub data: serde_json::Value,
}

impl Data {
    pub fn to_string(&self) -> Result<String, Error> {
        let string = serde_json::to_string(self)?;
        return Ok(string);
    }
}

pub struct Identity {
    pub hash: String,
    pub encryption_key: String,
    pub data: String,
}

#[derive(thiserror::Error, Debug)]
pub enum IdentityServiceError {
    #[error(transparent)]
    OpensslError(#[from] openssl::error::ErrorStack),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    
    #[error(transparent)]
    StdError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigServiceError {
    #[error(transparent)]
    ParseBoolError(#[from] std::str::ParseBoolError),

    #[error(transparent)]
    FromUtf8Error(#[from] std::str::Utf8Error),
    
    #[error(transparent)]
    StdError(#[from] std::io::Error),

    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),

    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError)
}


#[derive(thiserror::Error, Debug)]
pub enum StateServiceError {

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    
    #[error(transparent)]
    StdError(#[from] std::io::Error),

    #[error("`{0}`")]
    EncryptionKeyNotFound(String),
}

