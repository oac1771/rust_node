use ethers::providers::{Provider, Ws, StreamExt};

use std::sync::Arc;

use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::config::Config;
use crate::services::identity::IdentityService;

use crate::identifier::{Identifier, IdentifierEvents};


pub struct AuthenticationController {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub identity_service: IdentityService,
    pub contract: Identifier<Provider<Ws>>,
}

impl AuthenticationController {
    pub async fn new(config: Config) -> AuthenticationController {
        
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let identity_service = IdentityService::new();
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

    async fn foo(&self) {

    }

    pub async fn listen(&self) {
        
        let events = self.contract.events();

        // tokio::spawn(async move {
        //     println!("Starting PubSub Thread...");

        //     let mut stream = events.subscribe().await.unwrap();
    
        //     while let Some(Ok(event)) = stream.next().await {
        //         println!("Event: {:?}", event);
        //         match event {
        //             IdentifierEvents::AuthenticationRequestFilter(request) => {self.foo();},
        //             _ => {}
        //         }
        //     }
        // });

    }
}