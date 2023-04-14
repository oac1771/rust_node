use futures::{future::BoxFuture, FutureExt};

use reqwest;

use rocket::serde::Serialize;
use rocket::serde::json::Json;

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

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
