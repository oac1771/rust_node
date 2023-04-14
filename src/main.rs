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

#[post("/add/<file_name>")]
async fn add(file_name: &str) -> Json<client::request_client::Response> {

    let client = client::request_client::RequestClient::new();
    let response = client.post_multipart("http://127.0.0.1:5001/api/v0/add", file_name).await;

    return response
}

// add pin rm endpoint http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#api-v0-pin-rm


#[post("/id")]
async fn id() -> Json<client::request_client::Response> {
    let client = client::request_client::RequestClient::new();
    let response = client.post("http://127.0.0.1:5001/api/v0/id").await;

    return response

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, add, id])
}