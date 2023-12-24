use std::sync::Arc;

use ethers::{
    contract::{stream::EventStream, ContractError, Event},
    providers::{Provider, StreamExt, SubscriptionStream, Ws},
    types::Log,
    utils::hex::encode,
};

use crate::clients::{
    ipfs::client::{IClient, IpfsClient},
    reqwest::client::ReqwestClient,
    zksync::contracts::identifier::{AuthenticationRequestFilter, Identifier, IdentifierEvents},
};

use crate::services::{
    config::Config,
    identity::{IdService, IdentityService},
    state::{StService, StateService},
};

use super::models::{AuthenticationError, AuthenticationResponse};

pub struct AuthenticationController<IC, S, I> {
    pub ipfs_client: IC,
    pub state_service: S,
    pub identity_service: I,
}

impl AuthenticationController<IpfsClient<ReqwestClient>, StateService, IdentityService> {
    pub async fn new(
        config: Config,
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

    pub async fn listen(config: Config) -> Result<(), AuthenticationError> {
        let ws_provider = Provider::<Ws>::connect(&config.zksync_config.zksync_ws_url).await?;

        let contract =
            Identifier::new(config.zksync_config.contract_address, Arc::new(ws_provider));

        let events = contract.events();

        let result = tokio::spawn(AuthenticationController::listen_for_events(events, config)).await?;

        return result;
    }

    pub async fn listen_for_events(
        events: Event<Arc<Provider<Ws>>, Provider<Ws>, IdentifierEvents>, config: Config
    ) -> Result<(), AuthenticationError> {
        match events.subscribe().await {
            Ok(mut stream) => {
                let authentication_controller = AuthenticationController::new(config).await;
                authentication_controller.start(&mut stream).await
            }
            Err(_) => {
                return Err(AuthenticationError {
                    err: "Unable to initiate event stream".to_string(),
                });
            }
        };

        return Ok(());
    }
}

impl<IC: IClient, S: StService, I: IdService> AuthenticationController<IC, S, I> {
    pub async fn start(
        &self,
        event_stream: &mut EventStream<
            '_,
            SubscriptionStream<'_, Ws, Log>,
            IdentifierEvents,
            ContractError<Provider<Ws>>,
        >,
    ) {
        while let Some(Ok(event)) = event_stream.next().await {
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

    pub async fn authenticate(
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
