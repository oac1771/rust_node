use serde_json;
use mockall_double::double;

#[double]
use crate::clients::reqwest::client::ReqwestClient;
#[allow(unused_imports)]
use crate::clients::reqwest::client::R;

use crate::clients::ipfs::models;

const IPFS_URL: &str = "http://127.0.0.1:5001";

pub struct IpfsClient {
    pub reqwest_client: ReqwestClient
}

impl IpfsClient{

    pub fn new() -> IpfsClient {
        let reqwest_client: ReqwestClient = ReqwestClient::new();
        let ipfs_client = IpfsClient {
            reqwest_client: reqwest_client
        };
        return ipfs_client
    }

    pub async fn get_id(&self) -> models::IpfsIdResponse {
        let url = format!("{}{}", IPFS_URL, "/api/v0/id");
        let response = self.reqwest_client.post(&url).await;
        let ipfs_response: models::IpfsIdResponse = serde_json::from_str(&response.body).unwrap();

        return ipfs_response
    }

    pub async fn add_file(&self, file_name: &str) -> models::IpfsAddFileResponse {
        let url = format!("{}{}", IPFS_URL, "/api/v0/add");
        let response = self.reqwest_client.post_multipart(&url, file_name).await;
        let ipfs_response: models::IpfsAddFileResponse = serde_json::from_str(&response.body).unwrap();

        return ipfs_response

    }

}