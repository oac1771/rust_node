use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub meta_data: String,
    pub data: serde_json::Value,
}

impl Data {
    pub fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
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