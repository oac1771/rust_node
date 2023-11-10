use ethers::{
    providers::{Provider, StreamExt, Ws},
    utils::hex::encode,
};

use std::sync::Arc;

use crate::clients::ipfs::{client::IpfsClient, models::IpfsClientError};
use crate::clients::zksync::client::ZksyncClient;
use crate::identifier::{AuthenticationRequestFilter, Identifier, IdentifierEvents};
use crate::services::{config::Config, identity::IdentityService, state::StateService, models::IdentityServiceError};

pub struct AuthenticationController {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub state_service: StateService,
    pub contract: Identifier<Provider<Ws>>,
    pub identity_service: IdentityService,
}

impl AuthenticationController {
    pub async fn new(config: &Config) -> AuthenticationController {
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let state_service = StateService {};
        let identity_service = IdentityService::new();

        let ws_provider = Provider::<Ws>::connect(config.zksync_config.zksync_ws_url.to_owned())
            .await
            .unwrap();
        let contract = Identifier::new(
            config.zksync_config.contract_address.to_owned(),
            Arc::new(ws_provider),
        );

        let authentication_controller = AuthenticationController {
            ipfs_client,
            zksync_client,
            state_service,
            contract,
            identity_service,
        };

        return authentication_controller;
    }

    pub async fn listen(&self) {
        let events = self.contract.events();
        let event_stream = events.subscribe().await;

        match event_stream {
            Ok(mut stream) => {
                println!("Starting EventStrem...");

                while let Some(Ok(event)) = stream.next().await {
                    println!("Event: {:?}", event);
                    match event {
                        IdentifierEvents::AuthenticationRequestFilter(request) => {
                            let authentication = self.authenticate(request).await;
                            match authentication {
                                Ok(()) => {
                                    println!("Authentication Successful!")
                                }
                                Err(err) => {
                                    println!("Authentication Unsuccessful: {:?}", err)
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                println!("Unable to start Event stream: {:?}", err);
            }
        }
    }

    async fn authenticate(
        &self,
        request: AuthenticationRequestFilter,
    ) -> Result<(), AuthenticationResponse> {

        let ipfs_data = self
            .ipfs_client
            .get(&request.ipfs_address)
            .await?;

        let principal_address = format!("0x{}", encode(request.principal));

        let encryption_key = self
            .state_service
            .get_encryption_key(&principal_address)
            .await.ok_or(AuthenticationResponse::DecryptionError(
                "Encryption Key is not in Saved State".to_string(),
            ))?;
            
        let identity = self.identity_service.regenerate_identity(&encryption_key, &ipfs_data.data)?;

        if identity.hash == request.data_hash {
            return Ok(());
        } else {
            return Err(AuthenticationResponse::HashMismatch(
                "Hashes do not match".to_string(),
            ));
        }


    }

}

#[derive(Debug)]
enum AuthenticationResponse {
    DecryptionError(String),
    IpfsGetError(String),
    HashMismatch(String),
}

impl From<IpfsClientError> for AuthenticationResponse {
    
    fn from(err: IpfsClientError) -> AuthenticationResponse {
        return AuthenticationResponse::IpfsGetError(err.err)
    }
}

impl From<IdentityServiceError> for AuthenticationResponse {
    
    fn from(err: IdentityServiceError) -> AuthenticationResponse {
        return AuthenticationResponse::DecryptionError(err.err)
    }
}