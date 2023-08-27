mod clients;
mod controllers;
mod services;
mod config;
mod state;
mod identifier;

#[macro_use] extern crate rocket;
use ethers::providers::Middleware;
use rocket::serde::json::Json;
use rocket::State;

use reqwest;

use clients::reqwest::models::Response;

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<controllers::models::Data>,
    config: &State<config::Config>,
    state: &State<state::State>,
    principal_address: &str) -> String {

    let register_controller = controllers::register::RegisterController::new(config, state).await;
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
        signers::{LocalWallet, Signer},
        middleware::SignerMiddleware
    };
    use std::{convert::TryFrom, sync::Arc};

    let principal: Address = "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92".parse().unwrap();
    let ipfs_address = "ipfs://foo".to_owned();
    let data_hash = "hash".to_owned();

    let http_provider = Provider::<Http>::try_from(&config.zksync_config.zksync_api_url).unwrap();
    let chain_id = http_provider.get_chainid().await.unwrap().as_u64();

    let wallet = config.zksync_config.private_key.parse::<LocalWallet>().unwrap().with_chain_id(chain_id);
    let http_provider = Provider::<Http>::try_from(&config.zksync_config.zksync_api_url).unwrap();
    let client = SignerMiddleware::new(http_provider, wallet);

    let identifier = identifier::Identifier::new(config.zksync_config.contract_address, Arc::new(&client));

    let ws_provider = Provider::<Ws>::connect(&config.zksync_config.zksync_ws_url).await.unwrap();
    let identifier_ws = identifier::Identifier::new(config.zksync_config.contract_address, Arc::new(&ws_provider));
    let events = identifier_ws.events();
    let mut stream = events.subscribe().await.unwrap();

    let foo = identifier.register_identity(principal, ipfs_address, data_hash);
    let tx = foo.send().await.unwrap().await.unwrap();
    println!("transaction: {:?}", tx.unwrap().transaction_hash);

    let token_id = identifier.get_current_token_id().call().await.unwrap();
    println!("current token id: {:?}", token_id);


    // let client = clients::zksync::client::ZksyncClient::new(&config.zksync_config).await;

    // let mut token_id = client.get_current_token_id().await;
    // println!("contract Address: {:?}", &config.zksync_config.contract_address);


    // client.register_identity(principal, ipfs_address, data_hash).await;

    // token_id = client.get_current_token_id().await;
    // println!("current token id after registration: {:?}", token_id);

    // let foo = identifier::Identifier::deploy(client.contract.client(), ()).unwrap().send().await.unwrap();
    // println!("{:?}", foo);
    // let bar = client.contract.owner_of(token_id-1).call().await.unwrap();

    // println!("the owner of token_id {:?} is {:?}", token_id-1, bar);
    // println!("ipfs address for token_id {:?}: {:?}", token_id-1, foo);

    
    // Abigen::new("Identifier", "contract/artifacts-zk/contracts/Identifier.sol/Identifier.json").unwrap().generate().unwrap().write_to_file("src/identifier.rs").unwrap();
    // let principal: Address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049".parse().unwrap();
    // let ipfs_address = "ipfs://foo".to_owned();
    // let data_hash = "hash".to_owned();
    // let contract_address: Address = config.zksync_config.contract_address.parse().expect("Invalid contract address");
    // let wallet: LocalWallet = config.zksync_config.private_key.parse().unwrap();

    // let mut provider = Provider::<Http>::try_from(&config.zksync_config.zksync_api_url).unwrap();
    // provider = provider.with_sender(wallet.address());
    // let ws_client = Provider::<Ws>::connect(&config.zksync_config.zksync_ws_url).await.unwrap();

    // let identifier = identifier::Identifier::new(contract_address, Arc::new(&provider));
    // let foo = identifier.get_current_token_id().call().await.unwrap();
    // println!("{:?}", foo);
    // let _bar = identifier.register_identity(principal, ipfs_address, data_hash).call().await.unwrap();


    // let identifier_ws = identifier::Identifier::new(contract_address, Arc::new(&ws_client));
    // let events = identifier_ws.events();
    // let mut stream = events.stream().await.unwrap();

    // while let Some(Ok(evt)) = stream.next().await {
    //     println!("{:?}", evt);
    // }


}

#[launch]
async fn rocket() -> _ {

    use ethers::{
        providers::{Provider, Http, Ws, StreamExt},
        types::Address,
        signers::{LocalWallet, Signer},
        middleware::SignerMiddleware
    };
    use std::{convert::TryFrom, sync::Arc};

    let config = config::get_config();
    let state = state::set_state();

    let contract_address_string = std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");
    let contract_address: Address = contract_address_string.parse().expect("Invalid contract address");
    let zksync_ws_url = std::env::var("ZKSYNC_WS_URL").expect("ZKSYNC_WS_URL not set");

    tokio::spawn(async move {
        println!("Starting Thread...");

        let ws_provider = Provider::<Ws>::connect(zksync_ws_url.to_owned()).await.unwrap();
        let identifier_ws = identifier::Identifier::new(contract_address.to_owned(), Arc::new(ws_provider));
        let events = identifier_ws.events();
        let mut stream = events.subscribe().await.unwrap();

        while let Some(Ok(evt)) = stream.next().await {
            println!("Inside thread {:?}", evt);
        }
    });

    rocket::build()
        .manage(config)
        .manage(state)
        .mount("/", routes![health, id, rm_pin, register, contract])
        
}