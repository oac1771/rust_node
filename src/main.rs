mod clients;
mod controllers;
mod services;
mod config;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use reqwest;

use clients::reqwest::models::Response;

#[get("/health")]
fn health() -> Json<Response> {

    Json(Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

#[post("/add", data = "<file_contents>")]
async fn add(file_contents: Json<controllers::models::FileContent>, config: &State<config::Config>) -> String {

    let add_controller = controllers::add::AddController::new(&config);
    let response = add_controller.add(file_contents.into_inner()).await;

    return response
}

#[post("/rm/<hash>")]
async fn rm_pin(hash: &str, config: &State<config::Config>) -> String {
    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config);
    let response = ipfs_client.rm_pin(hash).await;

    return response
}

#[post("/id")]
async fn id(config: &State<config::Config>) -> String {

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config);
    let response = ipfs_client.get_id().await;

    return response

}

#[launch]
fn rocket() -> _ {
    let config = config::get_config();
    rocket::build().mount("/", routes![health, id, rm_pin, add]).manage(config)
}