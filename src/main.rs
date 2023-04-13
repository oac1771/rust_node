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

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Disposition", reqwest::header::HeaderValue::from_str("form-data").unwrap());
    headers.insert("name", reqwest::header::HeaderValue::from_str("file").unwrap());
    headers.insert("filename", reqwest::header::HeaderValue::from_str("todo.txt").unwrap());


    let file = File::open("todo.txt").await.unwrap();

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    // let client = client::request_client::RequestClient::new();
    // let response = client.post("http://127.0.0.1:5001/api/v0/add", Some(body)).await;

    // return response

    let client = reqwest::Client::new();
    let request = client.post("http://127.0.0.1:5001/api/v0/add").body(body).headers(headers).build().unwrap();

    for foo in request.body().into_iter() {
        println!("{:?}", foo)
    }
    println!("{:?}", request.headers());

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