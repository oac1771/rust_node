mod clients;
mod controllers;
mod services;
mod config;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use reqwest;

use clients::reqwest::models::Response;

#[post("/register", data = "<data>")]
async fn register(data: Json<controllers::models::Data>, config: &State<config::Config>) -> String {

    let register_controller = controllers::register::RegisterController::new(&config);
    let response = register_controller.register(data.into_inner()).await;

    return response
}

#[post("/rm/<hash>")]
async fn rm_pin(hash: &str, config: &State<config::Config>) -> String {
    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config);
    let response = ipfs_client.rm_pin(hash).await;

    return response
}

#[post("/id")]
async fn id(config: &State<config::Config>) -> String {

    let ipfs_client = clients::ipfs::client::IpfsClient::new(&config);
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

#[get("/get_encryption_keys")]
fn get_encryption_keys() -> String {

    use openssl::rsa::Rsa;
    use openssl::pkey::PKey;

    let rsa = Rsa::generate(2048).unwrap();
    let public_key = rsa.public_key_to_pem().unwrap();
    let private_key = rsa.private_key_to_pem().unwrap();

    // println!("Private Key: {:?}", String::from_utf8_lossy(&private_key));
    // println!("Pubblic Key: {:?}", String::from_utf8_lossy(&public_key));

    let recreated_rsa = PKey::private_key_from_pem(&String::from_utf8_lossy(&private_key).as_bytes().to_vec()).unwrap().rsa().unwrap();

    assert_eq!(recreated_rsa.private_key_to_pem().unwrap(), private_key);
    assert_eq!(recreated_rsa.public_key_to_pem().unwrap(), public_key);
 
    return "hi".to_string()
}

#[launch]
fn rocket() -> _ {
    let config = config::get_config();
    rocket::build().mount("/", routes![health, id, rm_pin, register, get_encryption_keys]).manage(config)
}