use serde::{Deserialize, Serialize, Deserializer};

// probalby dont need serialize

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IpfsIdResponse {
    pub ID: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct IpfsAddFileResponse {
    pub Name: String,
    pub Hash: String
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

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct IpfsClientErrorResponse {
    pub Body: String,
}