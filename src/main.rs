#[macro_use] extern crate rocket;
use reqwest;
use rocket::serde::json::Json;

mod client;

#[get("/health")]
fn health() -> Json<client::request_client::Response> {

    Json(client::request_client::Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

#[post("/add")]
async fn add() -> Json<client::request_client::Response> {
    let client = client::request_client::RequestClient::new();
    let response = client.post("http://127.0.0.1:5001/api/v0/add", None).await;

    return response
}

#[post("/id")]
async fn id() -> Json<client::request_client::Response> {
    let client = client::request_client::RequestClient::new();
    let response = client.post("http://127.0.0.1:5001/api/v0/id", None).await;

    return response

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, add, id])
}