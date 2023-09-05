use futures::{future::BoxFuture, FutureExt};
use async_trait::async_trait;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use serde::{de::DeserializeOwned, Serialize};
use crate::clients::reqwest::models::Error;

pub struct ReqwestClient {
    client: reqwest::Client
}

#[allow(dead_code)]
impl ReqwestClient {

    pub async fn post<H: DeserializeOwned + Serialize>(&self, url: &str) ->  Result<H, Error> {
        let request =  || async move {self.client.post(url).send().await}.boxed();
        let response = self.call::<H>(request).await;

        return response
    }

    pub async fn post_multipart<H: DeserializeOwned + Serialize>(&self, url: &str, file_name: &str) -> Result<H, Error>
    {

        let file = File::open(file_name).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());
        
        let body = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(body);
        let form = reqwest::multipart::Form::new().part("file", part);

        let request = || async move {self.client.post(url).multipart(form).send().await}.boxed();
        let response = self.call::<H>(request).await;


        return response

    }

    // this fn could also take type to handle deserializing into a specific Error type E and return that instead clients::reqwest::models::Error
    // check if from_str().unwrap fails (add match case) and add test to check for this
    pub async fn call<'a, H: DeserializeOwned + Serialize>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> 
    Result<H, Error>
    {

        let r = request().await;

        match r {
            Ok(req) => {
                match req.error_for_status() {
                    Ok(r) => {
                        let body = r.text().await.unwrap();
                        let deserialized_response: H = serde_json::from_str(&body).unwrap();

                        return Ok(deserialized_response)
                    },
                    Err(e) => {
                        let error = Error::new(e);
                        return Err(error)
                    }
                }
            }
            Err(err) => {
                let error = Error::new(err);
                return Err(error)
            }
        }

    }

    pub fn new() -> ReqwestClient {
        let client = reqwest::Client::new();
        let reqwest_client = ReqwestClient {
            client
        };
    
        return reqwest_client
    }
    
}


#[async_trait]
pub trait R {
    async fn post<H: DeserializeOwned + Serialize + 'static>(&self, url: &str) -> Result<H, Error>;
    async fn post_multipart<H: DeserializeOwned + Serialize + 'static>(&self, url: &str, file_name: &str) -> Result<H, Error>;
}

#[cfg(test)]
use mockall::mock;

#[cfg(test)]
mock!{
    pub ReqwestClient{}

    #[async_trait]
    impl R for ReqwestClient {
        async fn post<H: DeserializeOwned + Serialize + 'static>(&self, url: &str) -> Result<H, Error>;
        async fn post_multipart<H: DeserializeOwned + Serialize + 'static>(&self, url: &str, file_name: &str) -> Result<H, Error>;
    }
}
