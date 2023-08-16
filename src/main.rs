mod clients;
mod controllers;
mod services;
mod config;
mod identifier;

#[macro_use] extern crate rocket;
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
        types::{Address, U256},
        contract::{Contract, EthEvent},
        abi
    };
    use ethers_signers::LocalWallet;
    use ethers_contract_abigen::Abigen;
    use std::{convert::TryFrom, sync::Arc};
    use serde::Deserialize;
    use tokio::fs;

    #[derive(Deserialize)]
    struct AbiFilter {
        abi: Vec<serde_json::Value>
    }

    #[derive(Debug, Clone, EthEvent)]
    pub struct Transfer {
        pub from: Address,
        pub to: Address,
        pub token_id: U256,
    }


    let principal: Address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049".parse().unwrap();
    let ipfs_address = "ipfs://foo".to_owned();
    let data_hash = "hash".to_owned();


    let client = Provider::<Http>::try_from(&config.zksync_config.zksync_url).unwrap();
    let contract_address: Address = config.zksync_config.contract_address.parse().expect("Invalid contract address");
    let wallet: LocalWallet = config.zksync_config.private_key.parse().unwrap();

    Abigen::new("Identifier", "contract/artifacts-zk/contracts/Identifier.sol/Identifier.json").unwrap().generate().unwrap().write_to_file("src/identifier.rs").unwrap();

    let identifier = identifier::Identifier::new(contract_address, Arc::new(&client));
    let foo = identifier.get_current_token_id().call().await.unwrap();
    println!("{:?}", foo);
    let bar = identifier.register_identity(principal, ipfs_address, data_hash).call().await.unwrap();

    let ws_client = Provider::<Ws>::connect("ws://localhost:3051").await.unwrap();

    let event = Contract::event_of_type::<Transfer>(Arc::new(&ws_client));
    let mut stream = event.subscribe_with_meta().await.unwrap();


    // let abi_string = fs::read_to_string("contract/artifacts-zk/contracts/Identifier.sol/Identifier.json").await.unwrap();
    // let abi_filter: AbiFilter = serde_json::from_str(&abi_string).unwrap();
    // let abi: abi::Abi = serde_json::from_str(&serde_json::to_string(&abi_filter.abi).unwrap()).unwrap();
    // let contract = Contract::new(contract_address, abi, Arc::new(&ws_client));
    // let event = contract.event::<Transfer>();

    // while let Some(Ok((log, meta))) = stream.next().await {
    //     println!("{log:?}");
    //     println!("{meta:?}")
    // }

    loop {
        match stream.next().await {
            Some(Ok((log, meta))) => {
                println!("new event: {:?}", log);
                println!("new event metadata: {:?}", meta);
            }
            _ => {
                println!("not found")
            }
        }
    }

}

#[launch]
fn rocket() -> _ {
    let config = config::get_config();
    rocket::build().manage(config)
        .mount("/", routes![health, id, rm_pin, register, contract])
}