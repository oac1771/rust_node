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

    pub async fn post<D, E>(&self, url: &str) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error>
    {
        let request = || async move { self.client.post(url).send().await }.boxed();
        let response = self.call::<D, E>(request).await;

        return response;
    }

    pub async fn post_multipart<D, E>(&self, url: &str, file_path: &str) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error> + From<std::io::Error>
    {
        let file: File = File::open(file_path).await?;
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
        let response = self.call::<D, E>(request).await;

        return response;
    }

    pub async fn call<'a, D, E>(
        &self,
        request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>,
    ) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error>
    {
        let response = request().await?;
        let resp = response.error_for_status()?;
        let body = resp.text().await?;
        let r = serde_json::from_str::<D>(&body)?;

        return Ok(r);
    }
}

#[cfg(test)]
use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;

#[cfg(test)]
#[async_trait]
pub trait R {
    async fn post<D: 'static, E: 'static>(&self, url: &str) -> Result<D, E>;
    async fn post_multipart<D: 'static, E: 'static>(&self, url: &str, file_path: &str) -> Result<D, E>;
}

#[cfg(test)]
mock! {
    pub ReqwestClient{}
    #[async_trait]
    impl R for ReqwestClient {
        async fn post<D: 'static, E: 'static>(&self, url: &str) -> Result<D, E>;
        async fn post_multipart<D: 'static, E: 'static>(&self, url: &str, file_path: &str) -> Result<D, E>;
    }
}
