use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct IpfsIdResponse {
    pub ID: String,
}
