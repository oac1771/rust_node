use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IpfsIdResponse {
    pub ID: String,
}
