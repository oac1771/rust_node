use crate::clients::reqwest::models::Error;
use futures::{future::BoxFuture, FutureExt};
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct ReqwestClient {
    client: reqwest::Client,
}


impl ReqwestClient {
    pub fn new() -> ReqwestClient {
        let client = reqwest::Client::new();
        let reqwest_client = ReqwestClient { client };

        return reqwest_client;
    }

    pub async fn post<'a, R>(&self, url: &str) -> Result<R, Error>
    where
        R: Deserialize<'a>,
    {
        let request = || async move { self.client.post(url).send().await }.boxed();
        let response = self.call(request).await?;
        let r = self.handle(&response).await;

        return r;
    }

    pub async fn post_multipart<'a, R>(&self, url: &str, file_path: &str) -> Result<R, Error>
    where
        R: Deserialize<'a>,
    {
        let file: File = File::open(file_path).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());

        let body = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(body);
        let form = reqwest::multipart::Form::new().part("file", part);

        let request = || {
            async move {
                self.client
                    .post(url)
                    .multipart(form)
                    .header("Content-Type", "application/octet-stream")
                    .send()
                    .await
            }
            .boxed()
        };
        let response = self.call(request).await?;
        let r = self.handle(&response).await;

        return r;
    }

    pub async fn call<'a>(
        &self,
        request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>,
    ) -> Result<String, Error> {
        let request = request().await;

        match request {
            Ok(req) => match req.error_for_status() {
                Ok(r) => {
                    if let Ok(body) = r.text().await {
                        return Ok(body);
                    }

                    return Err(Error::new("unable to read text".to_string()));
                }
                Err(e) => {
                    let error = Error::new(e.to_string());
                    return Err(error);
                }
            },
            Err(err) => {
                let error = Error::new(err.to_string());
                return Err(error);
            }
        }
    }

    pub async fn handle<'de, R>(&self, response: &'de str) -> Result<R, Error>
    where
        R: Deserialize<'de>,
    {
        serde_json::from_str::<R>(response).map_err(|e| Error::new(e.to_string()))
    }
}

#[cfg(test)]
use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;

#[cfg(test)]
#[async_trait]
pub trait R {
    async fn post<D>(&self, url: &str) -> Result<D, Error>
    where
        D: 'static;
    async fn post_multipart<D>(&self, url: &str, file_path: &str) -> Result<D, Error>
    where
        D: 'static;
}

#[cfg(test)]
mock! {
    pub ReqwestClient{}
    #[async_trait]
    impl R for ReqwestClient {
        async fn post<D>(&self, url: &str) -> Result<D, Error>
        where
            D: 'static;
        async fn post_multipart<D>(&self, url: &str, file_path: &str) -> Result<D, Error>
        where
            D: 'static;
    }
}
