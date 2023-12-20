use ethers::{
    providers::{Provider, StreamExt, Ws},
    utils::hex::encode,
};

use crate::clients::{
    ipfs::{client::{IpfsClient, IClient}, models::IpfsClientError},
    reqwest::client::ReqwestClient,
    zksync::contracts::identifier::{AuthenticationRequestFilter, Identifier, IdentifierEvents}
};

use crate::services::{
    config::Config, identity::{IdentityService, IdService}, models::IdentityServiceError, state::{StateService, StService},
};

pub struct AuthenticationController<IC, S, I> {
    pub ipfs_client: IC,
    pub state_service: S,
    pub identity_service: I,
}

impl AuthenticationController<IpfsClient<ReqwestClient>, StateService, IdentityService> {
    pub async fn new(
        config: &Config,
    ) -> AuthenticationController<IpfsClient<ReqwestClient>, StateService, IdentityService> {

        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let state_service = StateService::new();
        let identity_service = IdentityService::new();

        let authentication_controller = AuthenticationController {
            ipfs_client,
            state_service,
            identity_service,
        };

        return authentication_controller;
    }
}

impl<IC: IClient, S: StService, I: IdService> AuthenticationController<IC, S, I> {

    pub async fn listen(&self, contract: Identifier<Provider<Ws>>) {
        let events = contract.events();
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
        let ipfs_data = self.ipfs_client.get(&request.ipfs_address).await?;

        let principal_address = format!("0x{}", encode(request.principal));

        let encryption_key = self
            .state_service
            .get_encryption_key(&principal_address)
            .await
            .ok_or(AuthenticationResponse::DecryptionError(
                "Encryption Key is not in Saved State".to_string(),
            ))?;

        let identity = self
            .identity_service
            .regenerate_identity(&encryption_key, &ipfs_data.data)?;

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
        return AuthenticationResponse::IpfsGetError(err.err);
    }
}

impl From<IdentityServiceError> for AuthenticationResponse {
    fn from(err: IdentityServiceError) -> AuthenticationResponse {
        return AuthenticationResponse::DecryptionError(err.err);
    }
}
