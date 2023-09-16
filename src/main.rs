mod clients;
mod controllers;
mod services;
mod config;
mod identifier;

use rocket::serde::Serialize;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

use controllers::{
    authentication::AuthenticationController,
    register::RegisterController,
    models::RegisterResponse
};

#[derive(Serialize)]
pub struct Health {
    pub status: String
}

#[get("/health")]
fn health() -> Json<Health> {

    Json(Health{
        status: "up".to_string() 
    })
}

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<services::models::Data>, 
    principal_address: &str) -> Json<RegisterResponse> {
    
    use ethers::{
        providers::{Provider, Http, Middleware},
        middleware::SignerMiddleware,
        signers::{LocalWallet, Signer},
    };

    use crate::identifier::Identifier;
    use std::{convert::TryFrom, sync::Arc};

    let config = config::read_config().await;
    
    let register_controller = RegisterController::new(&config).await;
    let response = register_controller.register(data.into_inner(), principal_address).await;



    let principal_private_key = "0xf12e28c0eb1ef4ff90478f6805b68d63737b7f33abfa091601140805da450d93";
    let zksync_client = clients::zksync::client::ZksyncClient::new(&config.zksync_config).await;

    let token_id = zksync_client.get_current_token_id().await;
    println!("current token id: {:?}", token_id);
    
    let http_provider = Provider::<Http>::try_from(&config.zksync_config.zksync_api_url).unwrap();
    let chain_id = http_provider.get_chainid().await.unwrap().as_u64();

    let wallet = principal_private_key.parse::<LocalWallet>().unwrap().with_chain_id(chain_id);
    let client = SignerMiddleware::new(http_provider, wallet);

    let contract = Identifier::new(config.zksync_config.contract_address, Arc::new(client));

    println!("authentiating...");
    let _foo = contract.authenticate(token_id - 1).send().await.unwrap().await.unwrap().unwrap();

    return Json(response)
}


#[delete("/remove/<principal_address>/<token_id>")]
async fn remove(principal_address: &str, token_id: u128) -> Json<RegisterResponse> {

    let config = config::read_config().await;

    let register_controller = RegisterController::new(&config).await;
    let response = register_controller.remove(principal_address, token_id).await;

    return Json(response)
}

#[post("/ipfs_id")]
async fn ipfs_id() -> String {

    let config = config::read_config().await;

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await.unwrap();

    return response.ID

}

#[post("/bootstrap/<contract_address>")]
async fn bootstrap(contract_address: &str) {

    config::set_contract_address(contract_address.to_string()).await;

    tokio::spawn(async move {
        let config = config::read_config().await;
        let authentication_controller = AuthenticationController::new(&config).await;

        authentication_controller.listen().await
        
    });
}



#[post("/foo", data = "<data>")]
async fn foo(data: Json<services::models::Data>) {

    use std::io::Write;

    let config = config::read_config().await;
    
    let identity_service = services::identity::IdentityService::new();
    let hash_service = services::hash::HashService::new();
    let encryption_service = services::encryption::EncryptionService::new();
    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);

    let mut identity_file = identity_service.generate_identity_file();
    let inner_data = data.into_inner();
    let hash = hash_service.hash(&inner_data.to_string());
    println!("content before encryption: {:?}", &inner_data.to_string());

    let (encrypted_content, priv_key) = encryption_service.encrypt(&inner_data.to_string());
    // println!("content after encryption: {:?}", encrypted_content);
    // println!("byte to string literal: {:?}", bytes_to_string_literal(&encrypted_content));

    // println!("content after encryption as string: {:?}", String::from_utf8_lossy(&encrypted_content).to_string());

    identity_file.write_all(&bytes_to_string_literal(&encrypted_content).as_bytes()).expect("Unable to write to tempfile");

    let file_contents = tokio::fs::read(identity_file.path().to_str().unwrap().to_string()).await.unwrap();
    // println!("content from temp file: {:?}", file_contents);
    // println!("content from temp file as string: {:?}", String::from_utf8_lossy(&file_contents).to_string());

    let identity_file_path = identity_file.path().to_str().unwrap().to_string();
    let add_file_response = ipfs_client.add_file(&identity_file_path).await.unwrap();
    // println!("ipfs address: {:?}", add_file_response.Hash);

    let get_file_response = ipfs_client.get(&add_file_response.Hash).await.unwrap();
    // println!("data from ipfs: {:?}", get_file_response);
    // println!("data from ipfs string literal to bytes: {:?}", string_literal_to_bytes(&get_file_response));

    let decoded_content = encryption_service.decrypt(string_literal_to_bytes(&get_file_response).unwrap(), priv_key);
    // println!("data from ipfs bytes to original string: {:?}", String::from_utf8_lossy(&decoded_content));

    let reconstructed_data: services::models::Data = serde_json::from_str(&String::from_utf8_lossy(&decoded_content)).unwrap();
    println!("reconstructed identity: {:?}", reconstructed_data.to_string());


}

#[launch]
async fn rocket() -> _ {

    config::create_config().await;
    services::state::StateService{}.create_state().await;

    rocket::build().mount("/", routes![health, register, remove, ipfs_id, bootstrap, foo])

}

fn bytes_to_string_literal(bytes: &[u8]) -> String {
    let mut result = String::from("[");
    
    for (index, byte) in bytes.iter().enumerate() {
        result.push_str(&byte.to_string());
        
        if index < bytes.len() - 1 {
            result.push(',');
        }
    }
    result.push(']');
    
    return result
}

fn string_literal_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1];        
        let bytes: Vec<u8> = inner
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .collect();
        
        return Some(bytes)
    } else {
        return None
    }
}