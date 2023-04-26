use serde::{Deserialize, Serialize};
use serde_json;

use crate::clients::reqwest::client;
use crate::clients::reqwest::client::R;

use std::sync::{Arc, Mutex};

const IPFS_URL: &str = "http://127.0.0.1:5001";

// reqwest client field should probably be wrapped around Arc(Mutex())
pub struct IpfsClient<'r> {
    reqwest_client: Box<dyn R + 'r>
}

impl<'r> IpfsClient<'r>{

    pub fn new() -> IpfsClient<'r> {
        let reqwest_client = client::create();
        let ipfs_client = IpfsClient {
            reqwest_client: Box::new(reqwest_client)
        };
        return ipfs_client
    }

    pub async fn get_id(&self) -> IpfsIdResponse {
        let url = format!("{}{}", IPFS_URL, "/api/v0/id");
        let response = self.reqwest_client.post(&url).await;
        let ipfs_response: IpfsIdResponse = serde_json::from_str(&response.body).unwrap();

        return ipfs_response
    }

}

#[derive(Deserialize, Serialize)]
pub struct IpfsIdResponse {
    pub ID: String,
}


#[cfg(test)]
#[path = "./test_client.rs"]
mod test_client;