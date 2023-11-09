use mockall_double::double;

use crate::clients::ipfs::models::*;
#[double]
use crate::clients::reqwest::client::ReqwestClient;
use crate::services::config::IpfsConfig;

#[allow(unused_imports)]
#[cfg(test)]
use crate::clients::reqwest::client::R;

pub struct IpfsClient {
    pub reqwest_client: ReqwestClient,
    pub ipfs_base_url: String,
}

impl IpfsClient {
    pub fn new(config: &IpfsConfig) -> IpfsClient {
        let reqwest_client: ReqwestClient = ReqwestClient::new();

        let ipfs_client = IpfsClient {
            reqwest_client: reqwest_client,
            ipfs_base_url: config.ipfs_base_url.to_string(),
        };
        return ipfs_client;
    }

    pub async fn get_id(&self) -> Result<IpfsIdResponse, IpfsClientError> {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/id");
        let response = self
            .reqwest_client
            .post::<IpfsIdResponse, IpfsClientError>(&url)
            .await;

        return response;
    }

    pub async fn add_file(&self, file_path: &str) -> Result<IpfsAddFileResponse, IpfsClientError> {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/add");
        let response = self
            .reqwest_client
            .post_multipart::<IpfsAddFileResponse, IpfsClientError>(&url, file_path)
            .await;

        return response;
    }

    pub async fn rm_pin(&self, hash: &str) -> Result<IpfsRemovePinResponse, IpfsClientError> {
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/pin/rm?arg=", hash);
        let response = self
            .reqwest_client
            .post::<IpfsRemovePinResponse, IpfsClientError>(&url)
            .await;

        return response;
    }

    pub async fn get(&self, hash: &str) -> Result<IpfsGetResponse, IpfsClientError> {
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/cat?arg=", hash);
        let response = self
            .reqwest_client
            .post::<IpfsGetResponse, IpfsClientError>(&url)
            .await;

        return response;
    }
}
