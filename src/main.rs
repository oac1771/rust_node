mod clients;
mod controllers;
mod services;
mod config;
mod state;
mod identifier;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;

use reqwest;

use clients::reqwest::models::Response;
use controllers::models::RegisterResponse;

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<controllers::models::Data>,
    config: &State<config::Config>,
    state: &State<state::State>,
    principal_address: &str) -> Json<RegisterResponse> {

    let register_controller = controllers::register::RegisterController::new(config, state).await;
    let response = register_controller.register(data.into_inner(), principal_address).await;

    return Json(response)
}


#[delete("/remove/<principal_address>/<token_id>")]
async fn remove(config: &State<config::Config>,
    state: &State<state::State>,
    principal_address: &str, token_id: u128) -> Json<RegisterResponse> {

    let register_controller = controllers::register::RegisterController::new(config, state).await;
    let response = register_controller.remove(principal_address, token_id).await;

    return Json(response)
}

#[post("/ipfs_id")]
async fn ipfs_id(config: &State<config::Config>) -> String {

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await.unwrap();

    return response.ID

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

    use ethers::types::{Address, Filter, Bytes};
    use ethers::providers::{Provider, Http};
    use ethers::contract::EthEvent;
    use ethers::abi::{Detokenize, Token, InvalidOutputType};
    use ethers::providers::Middleware;
    use ethers::utils::hex;

    use std::str;

    let principal_address =  "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92";
    let ipfs_address = "ipfs://foo".to_owned();
    let data_hash = "hash".to_owned();

    let zksync_client = clients::zksync::client::ZksyncClient::new(&config.zksync_config).await;
    zksync_client.register_identity(principal_address, &ipfs_address, &data_hash).await;
    let token_id = zksync_client.get_current_token_id().await;
    println!("current token id after registration: {:?}", token_id);

    let ipfs_addr = zksync_client.contract.get_ipfs_address(token_id-1).call().await.unwrap();
    println!("ipfs_addr: {}", zksync_client.from_bytes(ipfs_addr));

    let principal: Address = principal_address.parse().expect("Invalid principal address");
    let identity_removal_tx = zksync_client.contract.remove_identity(token_id-1, principal).send().await.unwrap().await.unwrap().unwrap().transaction_hash;
    println!("tx hash of identity removal: {:?}", identity_removal_tx);

    let http_provider = Provider::<Http>::try_from(&config.zksync_config.zksync_api_url).unwrap();
    let contract = zksync_client.contract;

    #[derive(Clone, Debug)]
    struct IpfsDeletionRequest {
        principal: Address,
    }

    impl Detokenize for IpfsDeletionRequest {

        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> 
        {
            for token in tokens {
                match token {
                    Token::Address(address) => {
                        let principal: Address = address.clone();
                        let event = IpfsDeletionRequest {principal};
                        return Ok(event)
                    },
                    _ => {return Err(InvalidOutputType("Invalid Principal token".to_string()))}
                }
            }

            return Err(InvalidOutputType("Invalid Principal token".to_string()))
        }
    }


    let filter = Filter::new().from_block(0).address(config.zksync_config.contract_address).event("IpfsDeletionRequest(bytes,address)");
    let logs = http_provider.get_logs(&filter).await.unwrap();

    println!("number of logs {}", logs.len());
    for log in logs {
        println!("log: {:?}", log);
        let event = contract.decode_event_raw("IpfsDeletionRequest", log.topics, log.data);
        println!("event: {:?}", event);

        match event {
            Ok(vec) => {
                for v in vec {
                    match v {
                        Token::Bytes(bytes) => {
                            println!("bytes to string: {:?}", str::from_utf8(&bytes).unwrap().to_string());
                        },
                        Token::Address(address) => {
                            println!("address: {:?}", address)
                        }
                        _ => {}
                    }
                }
            },
            Err(_) => {}
        }
        println!("------------------------------------");

    }


}

#[launch]
async fn rocket() -> _ {

    use ethers::{
        providers::{Provider, Ws, StreamExt},
        types::Address
    };
    use std::sync::Arc;

    let config = config::get_config();
    let state = state::set_state();

    // let contract_address_string = std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");
    // let contract_address: Address = contract_address_string.parse().expect("Invalid contract address");
    // let zksync_ws_url = std::env::var("ZKSYNC_WS_URL").expect("ZKSYNC_WS_URL not set");

    // tokio::spawn(async move {
    //     println!("Starting PubSub Thread...");

    //     let ws_provider = Provider::<Ws>::connect(zksync_ws_url.to_owned()).await.unwrap();
    //     let identifier_ws = identifier::Identifier::new(contract_address.to_owned(), Arc::new(ws_provider));
    //     let events = identifier_ws.events();
    //     let mut stream = events.subscribe().await.unwrap();

    //     while let Some(Ok(evt)) = stream.next().await {
    //         println!("PubSub Thread: {:?}", evt);
    //     }
    // });

    rocket::build()
        .manage(config)
        .manage(state)
        .mount("/", routes![health, register, remove, ipfs_id, contract])

}