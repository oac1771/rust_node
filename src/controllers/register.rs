use std::str;

use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

use crate::clients::{
    ipfs::client::IpfsClient,
    zksync::{
        client::{ZClient, ZksyncClient},
        contracts::identifier::Identifier,
    },
};
use crate::services::{
    config::Config, identity::IdentityService, models::Data, state::StateService,
};
use crate::{
    clients::{ipfs::client::IClient, reqwest::client::ReqwestClient},
    services::{identity::IdService, state::StService},
};

use super::models::{RegisterResponse, RegisterError, RemoveResponse};

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
    ) -> Result<
        RegisterController<
            IpfsClient<ReqwestClient>,
            ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>, Provider<Http>>,
            IdentityService,
            StateService,
        >,
        RegisterError,
    > {
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let identity_service = IdentityService::new();
        let zksync_client = ZksyncClient::new(&config.zksync_config).await?;

        let register_controller = RegisterController {
            ipfs_client,
            zksync_client,
            identity_service,
            state_service: StateService {},
            check_identity: config.check_identity,
        };
        return Ok(register_controller);
    }
}
impl<IC: IClient, ZC: ZClient, IS: IdService, SS: StService> RegisterController<IC, ZC, IS, SS> {
    pub async fn register(
        &self,
        data: Data,
        principal_address: &str,
    ) -> Result<RegisterResponse, RegisterError> {
        if self.check_identity {
            if self.zksync_client.check_identity(principal_address).await? {
                let err_response = RegisterError {
                    err: "Identity already exists".to_string(),
                };
                return Err(err_response);
            }
        }

        let (identity_file, identity) = self.identity_service.create_identity(&data.to_string())?;
        let identity_file_path = identity_file.path().to_str().unwrap().to_string();

        let ipfs_response = self.ipfs_client.add_file(&identity_file_path).await?;
        let tx_hash = self
            .zksync_client
            .register_identity(principal_address, &ipfs_response.Hash, &identity.hash)
            .await?;
        let token_id = self.zksync_client.get_token_id(principal_address).await?;
        self.state_service
            .save_encryption_key(principal_address, &identity.encryption_key)
            .await;

        let register_response = RegisterResponse::new(
            tx_hash,
            token_id,
            ipfs_response.Hash,
            identity.encryption_key,
        )?;

        return Ok(register_response);
    }

    pub async fn remove(
        &self,
        principal_address: &str,
        token_id: u128,
    ) -> Result<RemoveResponse, RegisterError> {
        if self.check_identity {
            if !self.zksync_client.check_identity(principal_address).await? {
                let err_response = RegisterError {
                    err: "Identity does not exist".to_string(),
                };
                return Err(err_response);
            };
        }

        let tx_hash = self
            .zksync_client
            .remove_identity(principal_address, token_id)
            .await?;
        let ipfs_addr = self
            .zksync_client
            .get_ipfs_addr(principal_address, token_id)
            .await?;

        match ipfs_addr {
            Some(address) => {
                let ipfs_response = self.ipfs_client.rm_pin(&address).await?;
                let response = RemoveResponse::new(tx_hash, ipfs_response.Pins);
                return Ok(response)
            }
            _ => {
                return Err(RegisterError{err: "Ipfs Address not found".to_string()})
            }
        }
    }
}
