#[macro_use] extern crate rocket;
use reqwest;
use rocket::serde::json::Json;

mod client;

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[get("/health")]
fn health() -> Json<client::request_client::Response> {

    Json(client::request_client::Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

#[post("/add")]
async fn add() {

    let file = File::open("todo.txt").await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    let part = reqwest::multipart::Part::stream(body);
    let form = reqwest::multipart::Form::new().part("file", part);
    let client = reqwest::Client::new();

    let response = client.post("http://127.0.0.1:5001/api/v0/add").multipart(form).send().await.unwrap();
    println!("{:?}", response.text().await.unwrap());

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