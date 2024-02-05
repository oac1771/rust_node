mod clients;
mod controllers;
mod messages;
mod payloads;
mod services;
mod utils;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
// use axum_macros::debug_handler;

// use clients::ipfs::{
//     client::IClient,
//     models::{IpfsClientError, IpfsIdResponse},
// };

#[allow(unused_imports)]
use controllers::{
    models::{AuthenticationError, Health, RegisterError, RegisterResponse, RemoveResponse},
    // authentication::AuthenticationController,
    register::RegisterController,
};

use messages::{AppError, AppResponse};
use payloads::Register;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/register", post(register));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> (StatusCode, Json<Health>) {
    let response = Health {
        status: "Up".to_string(),
    };

    return (StatusCode::OK, Json(response));
}

// #[debug_handler]
async fn register(
    Json(payload): Json<Register>,
) -> Result<AppResponse<RegisterResponse>, AppError> {
    let config = services::config::read_config().await?;

    let register_controller = RegisterController::new(&config).await?;

    let response = register_controller
        .register(payload.data, &payload.principal_address)
        .await?;
    return Ok(AppResponse(response));
}

// async fn bootstrap(Json(payload): Json<BootStrap>) -> Result<(), Json<AuthenticationError>> {
//     services::config::set_contract_address(&payload.contract_address).await?;
//     services::state::create_state().await?;

//     let config = services::config::read_config().await?;
//     AuthenticationController::listen(config).await?;

//     return Ok(());
// }

// // #[delete("/remove/<principal_address>/<token_id>")]
// async fn remove(
//     principal_address: &str,
//     token_id: u128,
// ) -> Result<Json<RemoveResponse>, Json<RegisterError>> {
//     let config = services::config::read_config().await?;

//     let register_controller = RegisterController::new(&config).await?;
//     let response = register_controller
//         .remove(principal_address, token_id)
//         .await?;

//     return Ok(Json(response));
// }

// // #[post("/ipfs_id")]
// async fn ipfs_id() -> Result<Json<IpfsIdResponse>, Json<IpfsClientError>> {
//     let config = services::config::read_config().await?;

//     let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
//     let response = ipfs_client.get_id().await?;

//     return Ok(Json(response));
// }
