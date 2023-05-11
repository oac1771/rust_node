use serde_json;
use mockall_double::double;

use crate::clients::ipfs::models::*;
use crate::clients::reqwest::models::*;
#[double]
use crate::clients::reqwest::client::ReqwestClient;
#[allow(unused_imports)]
use crate::clients::reqwest::client::R;

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
    // move error handling stuff into general function that all methods will call
    // look into function that takes type and uses that type in serde_json::from_str and returns instance of that Type
        // might not enum in this case, would return Trait, but Json() thing in main.rs might not work on trait :( 
            // actually could have two enum variants: A successfull and error one, where successful takes Trait data (if possible)

    pub async fn get_id(&self) -> IpfsClientResponse {
        let url = format!("{}{}", IPFS_URL, "/api/v0/id");
        let response = self.reqwest_client.post(&url).await;

        match response {
            Ok(req) => {
                let ipfs_response: IpfsIdResponse = serde_json::from_str(&req.body).unwrap();
                return IpfsClientResponse::IdResponse(ipfs_response)

            }
            Err(err) => {
                let ipfs_error_response = IpfsClientErrorResponse{Body: err.body};
                return IpfsClientResponse::ErrorResponse(ipfs_error_response)

            }
        }


    }

    // pub async fn add_file(&self, file_name: &str) -> IpfsAddFileResponse {
    //     let url = format!("{}{}", IPFS_URL, "/api/v0/add");
    //     let response = self.reqwest_client.post_multipart(&url, file_name).await;
    //     let ipfs_response: IpfsAddFileResponse = serde_json::from_str(&response.body).unwrap();

    //     return ipfs_response

    // }

    // pub async fn rm_pin(&self, hash: &str) -> IpfsRemovePinResponse {
    //     let url = format!("{}{}{}", IPFS_URL, "/api/v0/pin/rm?arg=", hash);
    //     let response = self.reqwest_client.post(&url).await;
    //     let ipfs_response: IpfsRemovePinResponse = serde_json::from_str(&response.body).unwrap();

    //     return ipfs_response
    // }

}