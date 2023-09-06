use ethers::providers::{Provider, Ws, StreamExt};

use std::sync::Arc;

use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::config::Config;
use crate::state::State;
use crate::services::identity::IdentityService;

use crate::identifier::{Identifier, IdentifierEvents};


pub struct AuthenticationController<'a> {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub identity_service: IdentityService<'a>,
    pub contract: Identifier<Provider<Ws>>,
}

impl<'a> AuthenticationController<'a> {
    pub async fn new(config: &'a Config, state: &'a State) -> AuthenticationController<'a> {
        
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let identity_service = IdentityService::new(&state);
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let ws_provider = Provider::<Ws>::connect(config.zksync_config.zksync_ws_url.to_owned()).await.unwrap();
        let contract = Identifier::new(config.zksync_config.contract_address.to_owned(), Arc::new(ws_provider));

        let authentication_controller = AuthenticationController {
            ipfs_client,
            zksync_client,
            identity_service,
            contract
        };

        return authentication_controller
    }

    pub async fn listen(&self) {
        
        let events = self.contract.events();

        tokio::spawn(async move {
            println!("Starting PubSub Thread...");
            let mut stream = events.subscribe().await.unwrap();
    
            while let Some(Ok(evt)) = stream.next().await {
                println!("PubSub Thread: {:?}", evt);
            }
        });

    }
}