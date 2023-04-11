use std::collections::HashMap;
use futures::{future::BoxFuture, FutureExt};

use reqwest;

use rocket::serde::Serialize;
use rocket::serde::json::Json;

pub struct RequestClient {
    client: reqwest::Client

}

impl RequestClient {

    pub fn new() -> RequestClient {
        let client = reqwest::Client::new();
        let request_client = RequestClient {
            client
        };

        return request_client
    }

    pub async fn post(&self, url: &str, data: Option<HashMap<&str, &str>>) -> Json<Response> {
        let request =  || async move {self.client.post(url).json(&data).send().await}.boxed();
        let response = self.call(request).await.unwrap();

        return response

    }

    async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> Result<Json<Response>,  ()> {

        let r = request().await;

        match r {
            Ok(r) => {
                let response = Response::new(r).await;
                return Ok(Json(response))
            },
            Err(_) => {
                println!("Error in call method");
                Err(())
            }
        }
    }
    
}


#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
    pub body: serde_json::Value,
    // pub content: String
}

// might need to do unwrap or else kind of thing to get json body if successful response or return string body if underlying error
impl Response {
    async fn new(r: reqwest::Response) -> Response {

        let response = Response {
            status_code: r.status().to_string(),
            body: r.json::<serde_json::Value>().await.unwrap(),
            // content: r.text().await.unwrap()
        };

        return response
    }
}
