use futures::{future::BoxFuture, FutureExt};

use rocket::serde::Serialize;
use rocket::serde::json::Json;

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct ReqwestClient {
    client: reqwest::Client
}

impl ReqwestClient {

    pub fn new() -> ReqwestClient {
        let client = reqwest::Client::new();
        let request_client = ReqwestClient {
            client
        };

        return request_client
    }

    pub async fn post(&self, url: &str) -> Json<Response> {
        let request =  || async move {self.client.post(url).send().await}.boxed();
        let response = self.call(request).await;

        return response
    }

    pub async fn post_multipart(&self, url: &str, file_name: &str) -> Json<Response> {

        let file = File::open(file_name).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());
        
        let body = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(body);
        let form = reqwest::multipart::Form::new().part("file", part);

        let request = || async move {self.client.post(url).multipart(form).send().await}.boxed();
        let response = self.call(request).await;

        return response

    }

    // add error handling if request throws error
    async fn call<'a, F>(&self, request: F) -> Json<Response> 
    where
        F: FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>
    {

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