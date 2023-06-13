use serde::{Deserialize, Serialize};
use serde_json;
use mockall_double::double;

use crate::clients::ipfs::models::*;
use crate::clients::reqwest::models::*;
#[double]
use crate::clients::reqwest::client::ReqwestClient;

#[allow(unused_imports)]
#[cfg(test)]
use crate::clients::reqwest::client::R;

pub struct IpfsClient {
    pub reqwest_client: ReqwestClient,
    pub ipfs_base_url: String
}

impl IpfsClient {

    pub fn new() -> IpfsClient {
        let reqwest_client: ReqwestClient = ReqwestClient::new();
        let ipfs_base_url = std::env::var("IPFS_BASE_URL").unwrap();
        println!("{:?}", ipfs_base_url);

        let ipfs_client = IpfsClient {
            reqwest_client: reqwest_client,
            ipfs_base_url: ipfs_base_url
        };
        return ipfs_client
    }

    pub fn handle<'a, H: Deserialize<'a> + Serialize>(&self, response: &'a Result<Response, Error>) -> String {
        match response {
            Ok(resp) => {
                let ipfs_response: H = serde_json::from_str(&resp.body).unwrap();
                let string_response = serde_json::to_string(&ipfs_response).unwrap();
                return string_response
            }
            Err(err) => {
                return err.body.clone()
            }
        }
    }

    pub async fn get_id(&self) -> String {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/id");
        let response = self.reqwest_client.post(&url).await;
         
        return self.handle::<IpfsIdResponse>(&response)

    }

    pub async fn add_file(&self, file_name: &str) -> String {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/add");
        let response = self.reqwest_client.post_multipart(&url, file_name).await;
        
        return self.handle::<IpfsAddFileResponse>(&response)

    }

    pub async fn rm_pin(&self, hash: &str) -> String {
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/pin/rm?arg=", hash);
        let response = self.reqwest_client.post(&url).await;
        return self.handle::<IpfsRemovePinResponse>(&response)

    }

}