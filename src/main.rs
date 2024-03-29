mod clients;
mod controllers;
mod messages;
mod payloads;
mod services;
mod utils;

use axum::{
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
    extract::Path
};

use clients::ipfs::{client::IClient, models::IpfsIdResponse};

use controllers::{
    models::{Health, RegisterResponse, RemoveResponse},
    authentication::AuthenticationController,
    register::RegisterController,
};

use messages::{AppError, AppResponse};
use payloads::{Register, Remove};

#[tokio::main]
async fn main() {
    services::config::create_config().await.unwrap();

    let app = Router::new()
        .route("/health", get(health))
        .route("/ipfs_id", get(ipfs_id))
        .route("/remove", delete(remove))
        .route("/bootstrap/:contract_address", post(bootstrap))
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

async fn bootstrap(Path(contract_address): Path<String>) -> Result<(), AppError>  {
    services::config::set_contract_address(&contract_address).await?;
    services::state::create_state().await?;

    let config = services::config::read_config().await?;
    AuthenticationController::listen(config).await?;

    return Ok(());
}

async fn remove(Json(payload): Json<Remove>) -> Result<AppResponse<RemoveResponse>, AppError> {
    let config = services::config::read_config().await?;

    let register_controller = RegisterController::new(&config).await?;
    let response = register_controller
        .remove(&payload.principal_address, payload.token_id)
        .await?;

    return Ok(AppResponse(response));
}

async fn ipfs_id() -> Result<AppResponse<IpfsIdResponse>, AppError> {
    let config = services::config::read_config().await?;

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await?;

    return Ok(AppResponse(response));
}
