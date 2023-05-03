mod clients;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use reqwest;

use clients::reqwest::models::Response;
use clients::ipfs::models::IpfsIdResponse;


#[get("/health")]
fn health() -> Json<Response> {

    Json(Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

#[post("/add/<file_name>")]
async fn add(file_name: &str) ->  Json<Response> {
    let client = clients::reqwest::client::ReqwestClient::new();
    let response = client.post_multipart("http://127.0.0.1:5001/api/v0/add", file_name).await;

    return Json(response)
}


// add pin rm endpoint http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#api-v0-pin-rm
#[post("/id")]
async fn id() -> Json<IpfsIdResponse>{

    let ipfs_client = clients::ipfs::client::IpfsClient::new();
    let response = ipfs_client.get_id().await;

    return Json(response)

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, id, add])
}