mod clients;
mod controllers;
mod services;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
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
async fn add(file_contents: Json<controllers::models::FileContent>) -> String {

    let add_controller = controllers::add::AddController::new();
    let response = add_controller.add(file_contents.into_inner()).await;

    return response
}

#[post("/rm/<hash>")]
async fn rm_pin(hash: &str) -> String {
    let ipfs_client = clients::ipfs::client::IpfsClient::new();
    let response = ipfs_client.rm_pin(hash).await;

    return response
}

#[post("/id")]
async fn id() -> String {

    let ipfs_client = clients::ipfs::client::IpfsClient::new();
    let response = ipfs_client.get_id().await;

    return response

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, id, rm_pin, add])
}