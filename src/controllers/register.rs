use serde_json::json;
use std::str;

use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::config::Config;
use crate::state::State;
use crate::services::identity::IdentityService;

use super::models::{Data, RegisterResponse};

pub struct RegisterController<'a> {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub identity_service: IdentityService<'a>
}

impl<'a> RegisterController<'a> {

    pub async fn new(config: &'a Config, state: &'a State) -> RegisterController<'a> {
        
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let identity_service = IdentityService::new(&state);
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let register_controller = RegisterController {
            ipfs_client,
            zksync_client,
            identity_service
        };
        return register_controller
    }

    pub async fn register(&self, data: Data, principal_address: &str) -> RegisterResponse {

        let check_identity = self.zksync_client.check_identity(principal_address).await;
        let mut register_response = RegisterResponse::new();

        // if check_identity {
        //     register_response.set_error("Identity already exists".to_string());
        //     return register_response
        // } 

        let mut identity_file = self.identity_service.generate_identity_file();
        let (hash, encryption_key) = self.identity_service.encrypt_file_contents(data, &mut identity_file);
        let identity_file_path = identity_file.path().to_str().unwrap().to_string();

        let response = self.ipfs_client.add_file(&identity_file_path).await;

        match response {
            Ok(ipfs_response) => {

                let tx_hash= self.zksync_client.register_identity(principal_address, &ipfs_response.Hash, &hash).await;
                let token_id = self.zksync_client.get_token_id(principal_address).await;
                self.identity_service.save_encryption_key(principal_address, &encryption_key);

                register_response.set_body(json!({
                    "tx_hash": tx_hash,
                    "token_id": token_id.unwrap().to_string(),
                    "ipfs_address": ipfs_response.Hash,
                    "encryption_key": &encryption_key
                }))
            },
            Err(err) => {
                register_response.set_error(err.body.to_string());
            }
        }


        return register_response

    }

    pub async fn remove(&self, principal_address: &str, token_id: u128) -> RegisterResponse {

        let mut register_response = RegisterResponse::new();
        let check_identity = self.zksync_client.check_identity(principal_address).await;

        // if !check_identity {
        //     register_response.set_error("Identity does not exist".to_string());
        //     return register_response
        // }

        let tx_hash = self.zksync_client.remove_identity(principal_address, token_id).await;
        let ipfs_addr = self.zksync_client.get_ipfs_addr(principal_address).await.unwrap();
        let foo = ipfs_addr.as_bytes();

        println!("as_bytes: {:?}", ipfs_addr.as_bytes());
        println!("to_string: {}", ipfs_addr);

        // let response = self.ipfs_client.rm_pin(hash).await.unwrap();

        return register_response
    }
}