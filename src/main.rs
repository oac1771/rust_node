use std::collections::HashMap;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use reqwest;

mod client;

#[get("/health")]
fn health() -> Json<client::request_client::Response> {

    Json(client::request_client::Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: serde_json::json!("")
    })
}

#[get("/add")]
fn add() {

}

#[post("/id")]
async fn id() -> Json<client::request_client::Response> {
    let data: HashMap<&str, &str> = HashMap::new();
    let client = client::request_client::RequestClient::new();
    let response = client.post(data, "http://127.0.0.1:5001/api/v0/id").await;

    return Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, add, id])
}