mod clients;

#[macro_use] extern crate rocket;
use clients::reqwest::client::ReqwestClient;
use rocket::serde::json::Json;
use reqwest;

use clients::reqwest::models::Response;
use clients::ipfs::models::*;


#[get("/health")]
fn health() -> Json<Response> {

    Json(Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

// refactor: make this take in json object, write file on the fly, add file to ipfs, delete local file
// #[post("/add/<file_name>")]
// async fn add(file_name: &str) ->  Json<IpfsAddFileResponse> {
//     let ipfs_client = clients::ipfs::client::IpfsClient::new();
//     let response = ipfs_client.add_file(file_name).await;

//     return Json(response)
// }

// #[post("/rm/<hash>")]
// async fn rm_pin(hash: &str) ->  Json<IpfsRemovePinResponse> {
//     let ipfs_client = clients::ipfs::client::IpfsClient::new();
//     let response = ipfs_client.rm_pin(hash).await;

//     return Json(response)
// }


#[post("/id")]
async fn id() {

    // let ipfs_client = clients::ipfs::client::IpfsClient::new();
    // let response = ipfs_client.get_id().await;

    let url = "http://127.0.0.1:5001/apiv0/id";
    let client = ReqwestClient::new();
    let response = client.post(&url).await;

    match response {
        Ok(req) => {
            println!("inside Ok block");
            println!("{:?}", req.body);

        }
        Err(err) => {
            println!("inside Err block");
            println!("{:?}", err.body);
        }
    }


}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, id])
}