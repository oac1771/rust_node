use serde::{Deserialize, Deserializer, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IpfsIdResponse {
    pub ID: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct IpfsAddFileResponse {
    pub Name: String,
    pub Hash: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct IpfsRemovePinResponse {
    pub Pins: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct IpfsGetResponse {
    pub data: String,
}

impl<'de> Deserialize<'de> for IpfsGetResponse {
    fn deserialize<D>(deserializer: D) -> Result<IpfsGetResponse, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = String::deserialize(deserializer)?;
        Ok(IpfsGetResponse { data })
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct IpfsClientError {
    pub err: String,
}

impl From<reqwest::Error> for IpfsClientError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for IpfsClientError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<std::io::Error> for IpfsClientError {
    fn from(error: std::io::Error) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl std::fmt::Display for IpfsClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}