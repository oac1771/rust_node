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
        let response = self.call(request).await;

        return response

    }

    async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> Json<Response> {

        let r = request().await.unwrap();
        let response = Response::new(r).await;

        return Json(response)
    }
    
}


#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
    pub body: String,
}

impl Response {
    async fn new(r: reqwest::Response) -> Response{

        let response = Response {
            status_code: r.status().to_string(),
            body: r.text().await.unwrap(),
        };

        return response
    }
}
