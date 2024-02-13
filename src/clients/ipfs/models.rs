use serde::{
    de::Visitor,
    {Deserialize, Deserializer, Serialize},
};
use thiserror::Error;

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
        return Self { data: Vec::new() };
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

#[derive(Error, Debug)]
pub enum IpfsClientError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error("Error deserializing http response")]
    SerdeDeserializeError(#[from] serde_json::Error),

    #[error("Error when reading file")]
    FileReadingError(#[from] std::io::Error),
}