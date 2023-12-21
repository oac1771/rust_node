use serde_json::json;
use std::str;

use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

use crate::clients::{
    ipfs::client::IpfsClient,
    zksync::{client::{ZksyncClient, ZClient}, contracts::identifier::Identifier},
};
use crate::services::{
    config::Config, identity::IdentityService, models::Data, state::StateService,
};
use crate::{
    clients::{ipfs::client::IClient, reqwest::client::ReqwestClient},
    services::{identity::IdService, state::StService},
};

use super::models::RegisterResponse;

pub struct RegisterController<IC, ZC, IS, SS> {
    pub ipfs_client: IC,
    pub zksync_client: ZC,
    pub identity_service: IS,
    pub state_service: SS,
    pub check_identity: bool,
}

impl
    RegisterController<
        IpfsClient<ReqwestClient>,
        ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>, Provider<Http>>,
        IdentityService,
        StateService,
    >
{
    pub async fn new(
        config: &Config,
    ) -> RegisterController<
        IpfsClient<ReqwestClient>,
        ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>, Provider<Http>>,
        IdentityService,
        StateService,
    > {
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let identity_service = IdentityService::new();
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let register_controller = RegisterController {
            ipfs_client,
            zksync_client,
            identity_service,
            state_service: StateService {},
            check_identity: config.check_identity,
        };
        return register_controller;
    }
}
impl<IC: IClient, ZC: ZClient, IS: IdService, SS: StService> RegisterController<IC, ZC, IS, SS> {
    pub async fn register(&self, data: Data, principal_address: &str) -> RegisterResponse {
        let mut register_response = RegisterResponse::new();

        if self.check_identity {
            if self.zksync_client.check_identity(principal_address).await {
                register_response.set_error("Identity already exists".to_string());
                return register_response
            }
        }

        let (identity_file, identity) = self
            .identity_service
            .create_identity(&data.to_string())
            .unwrap();

        let identity_file_path = identity_file.path().to_str().unwrap().to_string();
        let response = self.ipfs_client.add_file(&identity_file_path).await;

        match response {
            Ok(ipfs_response) => {
                let tx_hash = self
                    .zksync_client
                    .register_identity(principal_address, &ipfs_response.Hash, &identity.hash)
                    .await;
                let token_id = self.zksync_client.get_token_id(principal_address).await;
                self.state_service
                    .save_encryption_key(principal_address, &identity.encryption_key)
                    .await;

                register_response.set_body(json!({
                    "tx_hash": tx_hash,
                    "token_id": token_id.unwrap().to_string(),
                    "ipfs_address": ipfs_response.Hash,
                    "encryption_key": &identity.encryption_key
                }))
            }
            Err(err) => {
                register_response.set_error(err.err.to_string());
            }
        }

        return register_response;
    }

    pub async fn remove(&self, principal_address: &str, token_id: u128) -> RegisterResponse {
        let mut register_response = RegisterResponse::new();

        if self.check_identity {
            if self.zksync_client.check_identity(principal_address).await {
                register_response.set_error("Identity does not exist".to_string());
                return register_response
            };
        }

        let tx_hash = self
            .zksync_client
            .remove_identity(principal_address, token_id)
            .await;
        let ipfs_addr = self
            .zksync_client
            .get_ipfs_addr(principal_address, token_id)
            .await
            .unwrap()
            .to_string();

        let response = self.ipfs_client.rm_pin(&ipfs_addr).await;

        match response {
            Ok(ipfs_response) => register_response.set_body(json!({
                "tx_hash": tx_hash,
                "removed_pins": ipfs_response.Pins,
            })),
            Err(err) => {
                register_response.set_error(err.err.to_string());
            }
        }

        return register_response;
    }
}
