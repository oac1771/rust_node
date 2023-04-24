use futures::{future::BoxFuture, FutureExt};

use rocket::serde::Serialize;

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use async_trait::async_trait;

#[async_trait]
pub trait R {
    async fn post(&self, url: &str) -> Response;
    async fn post_multipart(&self, url: &str, file_name: &str) -> Response;
    // async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> Response;
}

pub struct ReqwestClient {
    client: reqwest::Client
}

#[async_trait]
impl R for ReqwestClient {

    async fn post(&self, url: &str) -> Response {
        let request =  || async move {self.client.post(url).send().await}.boxed();
        // let response = self.call(request);

        let r = request().await.unwrap();
        let response = Response::new(r).await;

        return response
    }

    async fn post_multipart(&self, url: &str, file_name: &str) -> Response {

        let file = File::open(file_name).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());
        
        let body = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(body);
        let form = reqwest::multipart::Form::new().part("file", part);

        let request = || async move {self.client.post(url).multipart(form).send().await}.boxed();
        // let response = self.call(request);

        let r = request().await.unwrap();
        let response = Response::new(r).await;

        return response

    }

    // add error handling if request throws error
    // async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> Response
    // {

    //     let r = request().await.unwrap();
    //     let response = Response::new(r).await;

    //     return response
    // }
    
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

pub fn create() -> ReqwestClient {
    let client = reqwest::Client::new();
    let reqwest_client = ReqwestClient {
        client
    };

    return reqwest_client
}