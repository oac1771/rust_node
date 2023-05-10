use serde_json;
use mockall_double::double;

#[double]
use crate::clients::reqwest::client::ReqwestClient;
#[allow(unused_imports)]
use crate::clients::reqwest::client::R;

use crate::clients::ipfs::models::*;

const IPFS_URL: &str = "http://127.0.0.1:5001";

pub struct IpfsClient {
    pub reqwest_client: ReqwestClient
}


// for error handling from reqwestclient, have a general method that returns IpfsClientError that has serder_json::Value
// and have all of these methods return either error or successfull response

// reqwest client will throw general Err type
impl IpfsClient{

    pub fn new() -> IpfsClient {
        let reqwest_client: ReqwestClient = ReqwestClient::new();
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

    pub async fn add_file(&self, file_name: &str) -> IpfsAddFileResponse {
        let url = format!("{}{}", IPFS_URL, "/api/v0/add");
        let response = self.reqwest_client.post_multipart(&url, file_name).await;
        let ipfs_response: IpfsAddFileResponse = serde_json::from_str(&response.body).unwrap();

        return ipfs_response

    }

    pub async fn rm_pin(&self, hash: &str) -> IpfsRemovePinResponse {
        let url = format!("{}{}{}", IPFS_URL, "/api/v0/pin/rm?arg=", hash);
        let response = self.reqwest_client.post(&url).await;
        let ipfs_response: IpfsRemovePinResponse = serde_json::from_str(&response.body).unwrap();

        return ipfs_response
    }

}