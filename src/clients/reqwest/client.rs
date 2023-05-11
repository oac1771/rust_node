use futures::{future::BoxFuture, FutureExt};
use async_trait::async_trait;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use crate::clients::reqwest::models::{Response, Error};

pub struct ReqwestClient {
    client: reqwest::Client
}

#[allow(dead_code)]
impl ReqwestClient {

    pub async fn post(&self, url: &str) ->  Result<Response, Error> {
        let request =  || async move {self.client.post(url).send().await}.boxed();
        let response = self.call(request).await;

        return response
    }

    // pub async fn post_multipart(&self, url: &str, file_name: &str) -> Result<Response, Error>
    // {

    //     let file = File::open(file_name).await.unwrap();
    //     let stream = FramedRead::new(file, BytesCodec::new());
        
    //     let body = reqwest::Body::wrap_stream(stream);
    //     let part = reqwest::multipart::Part::stream(body);
    //     let form = reqwest::multipart::Form::new().part("file", part);

    //     let request = || async move {self.client.post(url).multipart(form).send().await}.boxed();
    //     let response = self.call(request).await;


    //     return response

    // }

    pub async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> 
    Result<Response, Error>
    {

        let r = request().await;

        match r {
            Ok(req) => {
                match req.error_for_status() {
                    Ok(r) => {
                        let response = Response::new(r).await;
                        return Ok(response)
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
    async fn post(&self, url: &str) -> Result<Response, Error>;
    // async fn post_multipart(&self, url: &str, file_name: &str) -> Response;
}

#[cfg(test)]
use mockall::mock;
#[cfg(test)]
mock!{
    pub ReqwestClient{}

    #[async_trait]
    impl R for ReqwestClient {
        async fn post(&self, url: &str) -> Result<Response, Error>;
        // async fn post_multipart(&self, url: &str, file_name: &str) -> Response;
    }
}
