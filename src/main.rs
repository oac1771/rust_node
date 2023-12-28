mod clients;
mod controllers;
mod services;
mod utils;

use rocket::serde::Serialize;

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

use clients::ipfs::{
    client::IClient,
    models::{IpfsClientError, IpfsIdResponse},
};
use controllers::{
    authentication::AuthenticationController,
    models::{AuthenticationError, RegisterError, RegisterResponse, RemoveResponse},
    register::RegisterController,
};

#[derive(Serialize)]
pub struct Health {
    pub status: String,
}

#[get("/health")]
fn health() -> Json<Health> {
    Json(Health {
        status: "up".to_string(),
    })
}

#[post("/bootstrap/<contract_address>")]
async fn bootstrap(contract_address: &str) -> Result<(), Json<AuthenticationError>> {
    services::config::set_contract_address(contract_address).await;
    let config = services::config::read_config().await;
    AuthenticationController::listen(config).await?;

    return Ok(());
}

#[post("/register/<principal_address>", data = "<data>")]
async fn register(
    data: Json<services::models::Data>,
    principal_address: &str,
) -> Result<Json<RegisterResponse>, Json<RegisterError>> {
    let config = services::config::read_config().await;

    match RegisterController::new(&config).await {
        Ok(controller) => {
            let response = controller
                .register(data.into_inner(), principal_address)
                .await?;

            return Ok(Json(response));
        }
        Err(err) => return Err(Json(err)),
    }
}

#[delete("/remove/<principal_address>/<token_id>")]
async fn remove(
    principal_address: &str,
    token_id: u128,
) -> Result<Json<RemoveResponse>, Json<RegisterError>> {
    let config = services::config::read_config().await;

    match RegisterController::new(&config).await {
        Ok(controller) => {
            let response = controller.remove(principal_address, token_id).await?;

            return Ok(Json(response));
        }
        Err(err) => return Err(Json(err)),
    }
}

#[post("/ipfs_id")]
async fn ipfs_id() -> Result<Json<IpfsIdResponse>, Json<IpfsClientError>> {
    let config = services::config::read_config().await;

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await?;

    return Ok(Json(response));
}

#[launch]
async fn rocket() -> _ {
    services::config::create_config().await;
    services::state::StateService {}.create_state().await;

    rocket::build().mount("/", routes![health, ipfs_id, bootstrap, register, remove])
}
