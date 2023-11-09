use crate::clients::reqwest::models::Error;
use futures::{future::BoxFuture, FutureExt};
use serde::de::DeserializeOwned;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct ReqwestClient {
    client: reqwest::Client,
}

#[allow(dead_code)]
impl ReqwestClient {
    pub fn new() -> ReqwestClient {
        let client = reqwest::Client::new();
        let reqwest_client = ReqwestClient { client };

        return reqwest_client;
    }

    pub async fn post<R>(&self, url: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let request = || async move { self.client.post(url).send().await }.boxed();
        let response = self.call::<R>(request).await;

        return response;
    }

    pub async fn post_multipart<R>(&self, url: &str, file_path: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
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
        let response = self.call::<R>(request).await;

        return response;
    }

    pub async fn call<'a, R>(
        &self,
        request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let response = request().await.map_err(|e| Error::new(e.to_string()))?;
        let resp = response
            .error_for_status()
            .map_err(|e| Error::new(e.to_string()))?;
        let body = resp.text().await.map_err(|e| Error::new(e.to_string()))?;
        let r = serde_json::from_str::<R>(&body).map_err(|e| Error::new(e.to_string()));

        return r;
    }

    pub fn handle<R>(&self, response: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
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
