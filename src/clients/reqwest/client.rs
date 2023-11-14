use async_trait::async_trait;

use futures::{future::BoxFuture, FutureExt};
use serde::de::DeserializeOwned;
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

    pub async fn call<'a, D, E>(
        &self,
        request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>,
    ) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error>,
    {
        let response = request().await?;
        let resp = response.error_for_status()?;
        let body = resp.text().await?;
        let r = serde_json::from_str::<D>(&body)?;

        return Ok(r);
    }
}

#[async_trait]
impl Req for ReqwestClient {
    async fn post<D, E>(&self, url: &str) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error>,
    {
        let request = || async move { self.client.post(url).send().await }.boxed();
        let response = self.call::<D, E>(request).await;

        return response;
    }

    async fn post_multipart<D, E>(&self, url: &str, file_path: &str) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error> + From<std::io::Error>,
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
}

#[async_trait]
pub trait Req {
    async fn post<D, E>(&self, url: &str) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error>;
    async fn post_multipart<D, E>(&self, url: &str, file_path: &str) -> Result<D, E>
    where
        D: DeserializeOwned,
        E: From<reqwest::Error> + From<serde_json::Error> + From<std::io::Error>;
}

#[cfg(test)]
pub struct MockReqwestClient {
    expectations: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

#[cfg(test)]
impl MockReqwestClient {
    pub fn new() -> MockReqwestClient {
        return MockReqwestClient {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn expect_post<D, E>(&mut self) -> &mut Expectation<D, E>
    where
        D: DeserializeOwned + 'static,
        E: From<reqwest::Error> + From<serde_json::Error> + From<std::io::Error> + 'static,
    {
        self.expectations
            .entry("post".to_string())
            .or_insert_with(|| Box::new(Expectation::<D, E> { func: || {} }))
            .downcast_mut::<Expectation<D, E>>()
            .unwrap()
    }
}

#[cfg(test)]
// #[async_trait]
impl MockReqwestClient {
    async fn post<D, E>(&self, url: &str) -> Result<D, E>
    where
        D: DeserializeOwned + 'static,
        E: From<reqwest::Error> + From<serde_json::Error> + 'static,
    {
        let expectation = self
            .expectations
            .get("post")
            .unwrap()
            .downcast_ref::<Expectation<D, E>>()
            .unwrap();
        let result = (expectation.func)();

        return result;
    }
}

#[cfg(test)]
pub struct Expectation<D, E> {
    pub func: Box<dyn Fn() -> Result<D, E>>,
}

#[cfg(test)]
impl<
        D: DeserializeOwned + 'static,
        E: From<reqwest::Error> + From<serde_json::Error> + From<std::io::Error> + 'static,
    > Expectation<D, E>
{
    pub fn returns(&mut self, func: impl Fn() -> Result<D, E> + 'static) {
        self.func = Box::new(func);
    }
}
