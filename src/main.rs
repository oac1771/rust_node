#[macro_use] extern crate rocket;
use reqwest;
use rocket::serde::json::Json;

mod clients;
use crate::clients::reqwest::client::R;


#[get("/health")]
fn health() -> Json<clients::reqwest::client::Response> {

    Json(clients::reqwest::client::Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

#[post("/add/<file_name>")]
async fn add(file_name: &str) ->  Json<clients::reqwest::client::Response> {
    let client = clients::reqwest::client::create();
    let response = client.post_multipart("http://127.0.0.1:5001/api/v0/add", file_name).await;

    return Json(response)
}

// add pin rm endpoint http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#api-v0-pin-rm

#[post("/id")]
async fn id() -> Json<clients::ipfs::client::IpfsIdResponse>{

    let ipfs_client = Box::new(clients::ipfs::client::IpfsClient::new());
    let resposne = ipfs_client.get_id().await;

    return Json(resposne)

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, add])
}