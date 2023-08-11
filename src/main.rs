mod clients;
mod controllers;
mod services;
mod config;

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
        contract::Contract,
        providers::{Provider, Http},
        types::Address,
        abi
    };
    use ethers_signers::LocalWallet;
    use tokio::fs;
    use rocket::serde::Deserialize;
    use std::{convert::TryFrom, sync::Arc};


    #[derive(Deserialize)]
    struct AbiFilter {
        abi: Vec<serde_json::Value>
    }

    let client = Provider::<Http>::try_from(&config.zksync_config.zksync_url).unwrap();
    let contract_address: Address = config.zksync_config.contract_address.parse().expect("Invalid contract address");
    let wallet: LocalWallet = config.zksync_config.private_key.parse().unwrap();

    let abi_string = fs::read_to_string("contract/artifacts-zk/contracts/Identifier.sol/Identifier.json").await.unwrap();
    let abi_filter: AbiFilter = serde_json::from_str(&abi_string).unwrap();
    let abi: abi::Abi = serde_json::from_str(&serde_json::to_string(&abi_filter.abi).unwrap()).unwrap();

    let contract = Contract::new(contract_address, abi, Arc::new(client));

    let foo: abi::Uint = contract.method::<_, abi::Uint>("getCurrentTokenID", ()).unwrap().call().await.unwrap();
    println!("{:?}", foo);

}

#[launch]
fn rocket() -> _ {
    let config = config::get_config();
    rocket::build().manage(config)
        .mount("/", routes![health, id, rm_pin, register, contract])
}