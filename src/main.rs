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

const CONFIG_PATH: &str = "./var/config.json";

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<controllers::models::Data>, 
    principal_address: &str) -> Json<RegisterResponse> {

    let config = config::read_config(CONFIG_PATH).await;
    let register_controller = RegisterController::new(&config).await;
    let response = register_controller.register(data.into_inner(), principal_address).await;

    return Json(response)
}


#[delete("/remove/<principal_address>/<token_id>")]
async fn remove(principal_address: &str, token_id: u128) -> Json<RegisterResponse> {

    let config = config::read_config(CONFIG_PATH).await;
    let register_controller = RegisterController::new(&config).await;
    let response = register_controller.remove(principal_address, token_id).await;

    return Json(response)
}

#[post("/ipfs_id")]
async fn ipfs_id() -> String {

    let config = config::read_config(CONFIG_PATH).await;

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await.unwrap();

    return response.ID

}

#[get("/health")]
fn health() -> Json<Health> {

    Json(Health{
        status: "up".to_string() 
    })
}

#[get("/contract")]
async fn contract() {

    use ethers::types::Address;

    let config = config::read_config(CONFIG_PATH).await;

    let principal_address =  "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92";
    let ipfs_address = "ipfs://foo".to_owned();
    let data_hash = "hash".to_owned();

    let zksync_client = clients::zksync::client::ZksyncClient::new(&config.zksync_config).await;
    zksync_client.register_identity(principal_address, &ipfs_address, &data_hash).await;
    let token_id = zksync_client.get_current_token_id().await;
    println!("current token id after registration: {:?}", token_id);

    let ipfs_addr = zksync_client.contract.get_ipfs_address(token_id-1).call().await.unwrap();
    println!("ipfs_addr: {}", ipfs_addr);

    let principal: Address = principal_address.parse().expect("Invalid principal address");
    let identity_removal_tx = zksync_client.contract.remove_identity(token_id-1, principal).send().await.unwrap().await.unwrap().unwrap().transaction_hash;
    println!("tx hash of identity removal: {:?}", identity_removal_tx);

}

#[launch]
async fn rocket() -> _ {

    config::create_config(CONFIG_PATH).await;
    services::state::StateService{}.create_state().await;

    rocket::build().mount("/", routes![health, register, remove, ipfs_id, contract])

}

#[derive(Serialize)]
pub struct Health {
    pub status: String
}