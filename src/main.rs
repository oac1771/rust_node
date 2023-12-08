mod clients;
mod controllers;
mod services;
mod identifier;
mod utils;

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

#[post("/bootstrap/<contract_address>")]
async fn bootstrap(contract_address: &str) {

    services::config::set_contract_address(contract_address).await;

    tokio::spawn(async move {
        let config = services::config::read_config().await;
        let authentication_controller = AuthenticationController::new(&config).await;

        authentication_controller.listen().await
        
    });
}

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<services::models::Data>, 
    principal_address: &str) -> Json<RegisterResponse> {

    let config = services::config::read_config().await;
    
    let register_controller = RegisterController::new(&config).await;
    let response = register_controller.register(data.into_inner(), principal_address).await;

    return Json(response)
}

#[delete("/remove/<principal_address>/<token_id>")]
async fn remove(principal_address: &str, token_id: u128) -> Json<RegisterResponse> {

    let config = services::config::read_config().await;

    let register_controller = RegisterController::new(&config).await;
    let response = register_controller.remove(principal_address, token_id).await;

    return Json(response)
}

#[post("/ipfs_id")]
async fn ipfs_id() -> Json<crate::clients::ipfs::models::IpfsIdResponse> {

    let config = services::config::read_config().await;

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await.unwrap();

    return Json(response)

}


#[launch]
async fn rocket() -> _ {

    services::config::create_config().await;
    services::state::StateService{}.create_state().await;

    rocket::build().mount("/", routes![health, register, remove, ipfs_id, bootstrap])

}
