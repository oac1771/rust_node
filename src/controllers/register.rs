use serde_json::json;
use std::str;

use crate::clients::reqwest::client::ReqwestClient;
use crate::clients::{
    zksync::client::ZksyncClient,
    ipfs::client as ipfs_client,
};
use crate::services::{
    state::StateService,
    identity::IdentityService,
    models::Data,
    config::Config
};

use super::models::RegisterResponse;


// maybe put this check identity flag behind env var check read from config?
pub struct RegisterController {
    pub ipfs_client: ipfs_client::IpfsClient<ReqwestClient>,
    pub zksync_client: ZksyncClient,
    pub identity_service: IdentityService,
    pub state_service: StateService
}

impl RegisterController {

    pub async fn new(config: &Config) -> RegisterController{
        
        let ipfs_client = ipfs_client::new(&config.ipfs_config);
        let identity_service = IdentityService::new();
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let register_controller = RegisterController {
            ipfs_client,
            zksync_client,
            identity_service,
            state_service: StateService {}
        };
        return register_controller
    }

    pub async fn register(&self, data: Data, principal_address: &str) -> RegisterResponse {

        let mut register_response = RegisterResponse::new();

        // let check_identity = self.zksync_client.check_identity(principal_address).await;
        // if check_identity {
        //     register_response.set_error("Identity already exists".to_string());
        //     return register_response
        // } 

        let (identity_file, identity) = self.identity_service.create_identity(&data.to_string()).unwrap();
        
        let identity_file_path = identity_file.path().to_str().unwrap().to_string();
        let response = self.ipfs_client.add_file(&identity_file_path).await;

        match response {
            Ok(ipfs_response) => {

                let tx_hash= self.zksync_client.register_identity(principal_address, &ipfs_response.Hash, &identity.hash).await;
                let token_id = self.zksync_client.get_token_id(principal_address).await;
                self.state_service.save_encryption_key(principal_address, &identity.encryption_key).await;

                register_response.set_body(json!({
                    "tx_hash": tx_hash,
                    "token_id": token_id.unwrap().to_string(),
                    "ipfs_address": ipfs_response.Hash,
                    "encryption_key": &identity.encryption_key
                }))
            },
            Err(err) => {
                register_response.set_error(err.err.to_string());
            }
        }


        return register_response

    }

    pub async fn remove(&self, principal_address: &str, token_id: u128) -> RegisterResponse {

        let mut register_response = RegisterResponse::new();
        
        // let check_identity = self.zksync_client.check_identity(principal_address).await;
        // if !check_identity {
        //     register_response.set_error("Identity does not exist".to_string());
        //     return register_response
        // }

        let tx_hash = self.zksync_client.remove_identity(principal_address, token_id).await;
        let ipfs_addr = self.zksync_client.get_ipfs_addr(principal_address, token_id).await.unwrap().to_string();

        let response = self.ipfs_client.rm_pin(&ipfs_addr).await;

        match response {
            Ok(ipfs_response) => {

                register_response.set_body(json!({
                    "tx_hash": tx_hash,
                    "removed_pins": ipfs_response.Pins,
                }))
            },
            Err(err) => {
                register_response.set_error(err.err.to_string());
            }
        }

        return register_response

    }
}