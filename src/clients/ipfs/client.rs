use mockall_double::double;

use crate::clients::ipfs::models::*;
use crate::clients::reqwest::models::Error;
#[double]
use crate::clients::reqwest::client::ReqwestClient;

#[allow(unused_imports)]
#[cfg(test)]
use crate::clients::reqwest::client::R;
use crate::config::IpfsConfig;

pub struct IpfsClient {
    pub reqwest_client: ReqwestClient,
    pub ipfs_base_url: String
}

impl IpfsClient {

    pub fn new(config: &IpfsConfig) -> IpfsClient {
        let reqwest_client: ReqwestClient = ReqwestClient::new();

        let ipfs_client = IpfsClient {
            reqwest_client: reqwest_client,
            ipfs_base_url: config.ipfs_base_url.to_string()
        };
        return ipfs_client
    }

    pub async fn get_id(&self) -> Result<IpfsIdResponse, Error> {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/id");
        let response = self.reqwest_client.post(&url).await;

        if let Ok(body) = response {
            let r: IpfsIdResponse = serde_json::from_str(&body).unwrap();
            return Ok(r)
        };
         
        return Err(response.unwrap_err())

    }

    pub async fn add_file(&self, file_path: &str) -> Result<IpfsAddFileResponse, Error> {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/add");
        let response = self.reqwest_client.post_multipart(&url, file_path).await;

        if let Ok(body) = response {
            let r: IpfsAddFileResponse = serde_json::from_str(&body).unwrap();
            return Ok(r)
        };
         
        return Err(response.unwrap_err())

    }

    pub async fn rm_pin(&self, hash: &str) -> Result<IpfsRemovePinResponse, Error> {
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/pin/rm?arg=", hash);
        let response = self.reqwest_client.post(&url).await;

        if let Ok(body) = response {
            let r: IpfsRemovePinResponse = serde_json::from_str(&body).unwrap();
            return Ok(r)
        };
         
        return Err(response.unwrap_err())
        
    }

    pub async fn get(&self, hash: &str) -> Result<String, Error>{
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/cat?arg=", hash);
        let response = self.reqwest_client.post(&url).await;

        return response
        
    }

}