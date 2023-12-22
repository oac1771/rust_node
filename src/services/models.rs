use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub meta_data: String,
    pub data: serde_json::Value,
}

impl Data {
    pub fn to_string(&self) -> Result<String, Error> {
        let string = serde_json::to_string(self);
        return string;
    }
}

pub struct Identity {
    pub hash: String,
    pub encryption_key: String,
    pub data: String
}

#[derive(Debug)]
pub struct IdentityServiceError {
    pub err: String
}

impl From<openssl::error::ErrorStack> for IdentityServiceError {
    fn from(error: openssl::error::ErrorStack) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for IdentityServiceError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<std::io::Error> for IdentityServiceError {
    fn from(error: std::io::Error) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl std::fmt::Display for IdentityServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}