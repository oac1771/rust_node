mod clients;
mod controllers;
mod services;
mod config;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;

use reqwest;

use clients::reqwest::models::Response;

#[post("/register/<principal_address>", data = "<data>")]
async fn register(data: Json<controllers::models::Data>,
    config: &State<config::Config>,
    principal_address: &str) -> String {

    let register_controller = controllers::register::RegisterController::new(config);
    let response = register_controller.register(data.into_inner(), principal_address).await;

    return response
}


#[post("/rm/<hash>")]
async fn rm_pin(hash: &str, config: &State<config::Config>) -> String {
    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.rm_pin(hash).await;

    return response
}

#[post("/id")]
async fn id(config: &State<config::Config>) -> String {

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config.ipfs_config);
    let response = ipfs_client.get_id().await;

    return response

}

#[get("/health")]
fn health() -> Json<Response> {

    Json(Response{
        status_code: reqwest::StatusCode::OK.to_string(), 
        body: "".to_string()
    })
}

// #[get("/get_encryption_keys")]
// fn get_encryption_keys() {

//     use openssl::rsa::{Rsa, Padding};
//     use crate::services::models::FileContent;


//     let file_content = FileContent{
//         content: "content".to_string(),
//         hash: "hash".to_string()
//     };

//     let content = serde_json::to_string(&file_content).unwrap();
    
//     let rsa = Rsa::generate(2048).unwrap();

//     let mut encrypted_content = vec![0; rsa.size() as usize];
//     let encrypted_len = rsa.public_encrypt(content.as_bytes(), &mut encrypted_content, Padding::PKCS1).unwrap();
//     encrypted_content.truncate(encrypted_len);

//     let mut decrypted_content = vec![0; rsa.size() as usize];
//     let decrypted_len = rsa.private_decrypt(&encrypted_content, &mut decrypted_content, Padding::PKCS1).unwrap();
//     decrypted_content.truncate(decrypted_len);


//     println!("{:?}", String::from_utf8_lossy(&encrypted_content));
//     println!("{:?}", String::from_utf8_lossy(&decrypted_content).to_string());

// }

#[launch]
fn rocket() -> _ {
    let config = config::get_config();
    rocket::build().manage(config)
        .mount("/", routes![health, id, rm_pin, register])
}