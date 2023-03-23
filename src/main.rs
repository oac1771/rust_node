#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, Serialize, json::Json};
use reqwest;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Task<'r> {
    description: &'r str,
    complete: &'r str
}

#[derive(Serialize)]
struct Response {
    status_code: i32,
    description: String
}

#[get("/forward")]
fn forward() {
    let mut map = HashMap::new();
    map.insert("description", "foo");
    map.insert("complete", "yes");

    let client = reqwest::Client::new();
    async {
        let response = client.post("http://localhost:8001/receive").json(&map).send().await;
        match response.unwrap().status() {
            reqwest::StatusCode::OK => {
                println!("OK!")
            },
            _ => {
                println!("Uh oh! Something unexpected happened.");
            },
        }
    };

}

#[post("/receive", data = "<task>")]
fn receive(task: Json<Task<'_>>) -> Json<Response> {
    println!("Description: {}, Complete: {}", task.description, task.complete);
    Json(Response{
        status_code: 200,
        description: "Ok".to_string()
    })
}

#[get("/health")]
fn health() -> Json<Response> {
    Json(Response{
        status_code: 200,
        description: "Ok".to_string()
    })
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, forward, receive])
}