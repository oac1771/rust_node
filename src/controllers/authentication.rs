use ethers::{
    providers::{Provider, Ws, StreamExt},
    utils::hex::encode
};

use std::sync::Arc;

use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::config::Config;
use crate::services::{
    state::StateService,
    encryption::EncryptionService
};

use crate::identifier::{Identifier, 
    IdentifierEvents, 
    AuthenticationRequestFilter
};

pub struct AuthenticationController {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub state_service: StateService,
    pub encryption_service: EncryptionService,
    pub contract: Identifier<Provider<Ws>>,
}

impl AuthenticationController {

    pub async fn new(config: &Config) -> AuthenticationController {
        
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let state_service = StateService{};
        let encryption_service = EncryptionService::new();

        let ws_provider = Provider::<Ws>::connect(config.zksync_config.zksync_ws_url.to_owned()).await.unwrap();
        let contract = Identifier::new(config.zksync_config.contract_address.to_owned(), Arc::new(ws_provider));

        let authentication_controller = AuthenticationController {
            ipfs_client,
            zksync_client,
            state_service,
            encryption_service,
            contract
        };

        return authentication_controller
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
                            self.authenticate(request).await
                        },
                        _ => {}
                    }
                }
            },
            Err(err) => {
                println!("Unable to start Event stream: {:?}", err);
            }
        }



    }

    async fn authenticate(&self, request: AuthenticationRequestFilter) {
        let ipfs_data = self.ipfs_client.get(&request.ipfs_address).await;

        match ipfs_data {
            Ok(response) => {

                let principal_address = format!("0x{}", encode(request.principal));
                let encryption_key = self.state_service.get_encryption_key(&principal_address).await;
                
                let data = response.as_bytes().to_vec();
                let decrypted_data = self.encryption_service.decrypt(data, encryption_key);

            },
            Err(err) => {
                println!("Error: {:?}", err.body);
            }
        }

    }

}