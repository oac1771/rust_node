#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use reqwest;

mod client;

#[get("/health")]
fn health() -> Json<client::request_client::Response> {

    Json(client::request_client::Response{
        status_code: reqwest::StatusCode::OK.to_string()
    })
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health])
}