use crate::clients::reqwest::client::{self, R};

use serde::{Deserialize, Serialize};
use serde_json;

const IPFS_URL: &str = "http://127.0.0.1:5001";

pub struct IpfsClient {
    reqwest_client: client::ReqwestClient
}

impl IpfsClient{

    pub fn new() -> IpfsClient {
        let reqwest_client = client::create();
        let ipfs_client = IpfsClient {
            reqwest_client: reqwest_client
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