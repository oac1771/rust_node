use serde::{
    de::Visitor,
    {Deserialize, Deserializer, Serialize},
};

use crate::services::models::ConfigServiceError;

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

#[derive(Serialize)]
pub struct IpfsGetResponse {
    pub data: Vec<u8>,
}

impl IpfsGetResponse {
    fn new() -> Self {
        return Self {
            data: Vec::new()
        }
    }
}

impl<'de> Visitor<'de> for IpfsGetResponse {
    type Value = Self;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Parese Error")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::new();

        while let Some(element) = seq.next_element::<u8>()? {
            values.push(element);
        }

        return Ok(Self { data: values });
    }
}

impl<'de> Deserialize<'de> for IpfsGetResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = IpfsGetResponse::new();
        let response = deserializer.deserialize_seq(visitor)?;
        Ok(response)
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

// impl From<ConfigServiceError> for Json<IpfsClientError> {
//     fn from(value: ConfigServiceError) -> Self {
//         return Json(IpfsClientError{
//             err: value.to_string()
//         })
//     }
// }