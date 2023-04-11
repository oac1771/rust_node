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

    pub async fn post(&self, url: &str, data: Option<HashMap<&str, &str>>) -> Json<ResponseKind> {
        let request =  || async move {self.client.post(url).json(&data).send().await}.boxed();
        let response = self.call(request).await.unwrap();

        return response

    }

    async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> Result<Json<ResponseKind>,  ()> {
        let r = request().await;

        match r {
            Ok(r) => {
                match r.error_for_status() {
                    Ok(r) => {
                        let response = ResponseKind::Response(Response::new(r).await);
                        return Ok(Json(response))
                    },
                    Err(err) => {
                        let error = ResponseKind::Error(Error::new(err));
                        return Ok(Json(error))
                    }
                }
            },
            Err(_) => {
                println!("Error in response");
                Err(())
            }
        }
    }
    
}

#[derive(Serialize)]

pub enum ResponseKind {
    Response(Response),
    Error(Error)
}

#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
    pub body: serde_json::Value
}

impl Response {
    async fn new(r: reqwest::Response) -> Response {

        let response = Response {
            status_code: r.status().to_string(),
            body: r.json::<serde_json::Value>().await.unwrap()
        };

        return response
    }
}

#[derive(Serialize)]
pub struct Error {
    pub status_code: String,
}

// figure out how to get more error info here
impl Error {
    fn new(e: reqwest::Error) -> Error {

        let error = Error {
            status_code: e.status().unwrap().to_string()
        };

        return error

    }
}