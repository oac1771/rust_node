use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IpfsIdResponse {
    pub ID: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IpfsAddFileResponse {
    pub Name: String,
    pub Hash: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IpfsRemovePinResponse {
    pub Pins: Vec<String>,
}
