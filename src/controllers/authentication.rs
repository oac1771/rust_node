use ethers::{
    providers::{Provider, Ws, StreamExt},
    utils::hex::encode
};

use std::sync::Arc;

use crate::{utils::string_literal_to_bytes, clients::reqwest::models::Error};
use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::services::{
    state::StateService,
    encryption::EncryptionService,
    hash::HashService,
    config::Config
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
    pub hash_service: HashService,
    pub contract: Identifier<Provider<Ws>>,
}

impl AuthenticationController {

    pub async fn new(config: &Config) -> AuthenticationController {
        
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let state_service = StateService{};
        let encryption_service = EncryptionService::new();
        let hash_service = HashService::new();

        let ws_provider = Provider::<Ws>::connect(config.zksync_config.zksync_ws_url.to_owned()).await.unwrap();
        let contract = Identifier::new(config.zksync_config.contract_address.to_owned(), Arc::new(ws_provider));

        let authentication_controller = AuthenticationController {
            ipfs_client,
            zksync_client,
            state_service,
            hash_service,
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
                            let authentication = self.authenticate(request).await;
                            match authentication {
                                Ok(()) => {println!("Authentication Successful!")},
                                Err(err) => {println!("Authentication Unsuccessful: {:?}", err)}
                            }
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

    async fn authenticate(&self, request: AuthenticationRequestFilter) -> Result<(), Error> {
        let ipfs_data = self.ipfs_client.get(&request.ipfs_address).await?;
        let principal_address = format!("0x{}", encode(request.principal));

        let decrypted_data = self.decrypt_data(&principal_address, &ipfs_data).await;

        // let hash = self.hash_service.hash(&String::from_utf8(decrypted_data).unwrap());


        return Ok(())

        // match ipfs_data {
        //     Ok(response) => {

                // let principal_address = format!("0x{}", encode(request.principal));

                // if let Some(encryption_key) = self.state_service.get_encryption_key(&principal_address).await {

                //     if let Some(encrypted_bytes) = string_literal_to_bytes(&response) {
                //         let decryption = self.encryption_service.decrypt(encrypted_bytes, encryption_key);

                //         if let Ok(decrypted_data) = decryption {
                //             let hash = self.hash_service.hash(&String::from_utf8(decrypted_data).unwrap());
    
                //             if hash == request.data_hash {
                //                 println!("Authentication Successful!");
                //                 return Ok(())
                //             } else {
                //                 println!("Authentication unsuccessful...");
                //             }

                //         } else {
                //             println!("Decryption Error: {:?}", decryption)
                //         }
                //     } else {
                //         println!("String literal is not in byte form")
                //     }
    
                // } else {
                //     println!("Encryption key does not exist")
                // }

            // },
            // Err(err) => {
            //     println!("Error retreiving data from ipfs: {:?}", err.body);
            // }
        // }

    }

    async fn decrypt_data(&self, principal_address: &str, ipfs_data: &str) -> Result<String, AuthenticationResponse> {

        let encryption_key = self.state_service.get_encryption_key(principal_address).await;
        let encrypted_bytes = string_literal_to_bytes(ipfs_data);

        match (encryption_key, encrypted_bytes) {
            (Some(key), Some(bytes)) => {
                let decrypted_data = self.encryption_service.decrypt(bytes, key);
                // &String::from_utf8(decrypted_data).unwrap()
            },
            (Some(key), None) => {return Err(AuthenticationResponse::DecryptionError("encryption key does not exist".to_string()))},
            (None, Some(byte)) => {return Err(AuthenticationResponse::DecryptionError("Ipfs Data not in byte form".to_string()))},
            _ => {return Err(AuthenticationResponse::DecryptionError("Unable to retreive Key and Byte Data".to_string()))}
        }

    }

}

enum AuthenticationResponse {
    DecryptionError(String)
}