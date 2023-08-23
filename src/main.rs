mod clients;
mod controllers;
mod services;
mod config;
mod identifier;

#[macro_use] extern crate rocket;
use ethers_signers::Signer;
use rocket::serde::json::Json;
use rocket::State;

use reqwest;

use clients::reqwest::models::Response;

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<controllers::models::Data>,
    config: &State<config::Config>,
    principal_address: &str) -> String {

    let register_controller = controllers::register::RegisterController::new(config);
    let response = register_controller.register(data.into_inner(), principal_address).await;

    return response
}


#[post("/rm/<hash>")]
async fn rm_pin(hash: &str, config: &State<config::Config>) -> String {
    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.rm_pin(hash).await;

    return response
}

#[post("/id")]
async fn id(config: &State<config::Config>) -> String {

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await;

    return response

}

#[get("/health")]
fn health() -> Json<Response> {

    Json(Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

#[get("/contract")]
async fn contract(config: &State<config::Config>) {

    use ethers::{
        providers::{Provider, Http, Ws, StreamExt},
        types::Address,
    };
    use ethers_signers::LocalWallet;
    use ethers_contract_abigen::Abigen;
    use std::{convert::TryFrom, sync::Arc};

    
    Abigen::new("Identifier", "contract/artifacts-zk/contracts/Identifier.sol/Identifier.json").unwrap().generate().unwrap().write_to_file("src/identifier.rs").unwrap();
    let principal: Address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049".parse().unwrap();
    let ipfs_address = "ipfs://foo".to_owned();
    let data_hash = "hash".to_owned();
    let contract_address: Address = config.zksync_config.contract_address.parse().expect("Invalid contract address");
    let wallet: LocalWallet = config.zksync_config.private_key.parse().unwrap();

    let mut provider = Provider::<Http>::try_from(&config.zksync_config.zksync_api_url).unwrap();
    provider = provider.with_sender(wallet.address());
    let ws_client = Provider::<Ws>::connect(&config.zksync_config.zksync_ws_url).await.unwrap();

    let identifier = identifier::Identifier::new(contract_address, Arc::new(&provider));
    let foo = identifier.get_current_token_id().call().await.unwrap();
    println!("{:?}", foo);
    let _bar = identifier.register_identity(principal, ipfs_address, data_hash).call().await.unwrap();

    let identifier_ws = identifier::Identifier::new(contract_address, Arc::new(&ws_client));
    let events = identifier_ws.events();

    let mut stream = events.stream().await.unwrap();

    while let Some(Ok(evt)) = stream.next().await {
        println!("{:?}", evt);
    }


}

#[launch]
fn rocket() -> _ {
    let config = config::get_config();
    rocket::build().manage(config)
        .mount("/", routes![health, id, rm_pin, register, contract])
}