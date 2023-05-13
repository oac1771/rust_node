mod clients;

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

// refactor: make this take in json object, write file on the fly, add file to ipfs, delete local file
#[post("/add/<file_name>")]
async fn add(file_name: &str) -> String {
    let ipfs_client = clients::ipfs::client::IpfsClient::new();
    let response = ipfs_client.add_file(file_name).await;

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