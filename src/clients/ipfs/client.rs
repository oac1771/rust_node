use super::models::*;
use crate::services::config::IpfsConfig;

use crate::clients::reqwest::client::{Req, ReqwestClient};

#[cfg(test)]
use serde::de::DeserializeOwned;

#[async_trait]
pub trait IClient {
    async fn get_id(&self) -> Result<IpfsIdResponse, IpfsClientError>;
    async fn add_file(&self, file_path: &str) -> Result<IpfsAddFileResponse, IpfsClientError>;
    async fn rm_pin(&self, hash: &str) -> Result<IpfsRemovePinResponse, IpfsClientError>;
    async fn get(&self, hash: &str) -> Result<IpfsGetResponse, IpfsClientError>;
}

pub struct IpfsClient<R> {
    pub req: R,
    pub ipfs_base_url: String,
}

impl IpfsClient<ReqwestClient> {
    pub fn new(config: &IpfsConfig) -> IpfsClient<ReqwestClient> {
        let req = ReqwestClient::new();

        let ipfs_client = IpfsClient {
            req: req,
            ipfs_base_url: config.ipfs_base_url.to_string(),
        };
        return ipfs_client;
    }
}

#[async_trait]
impl<R: Req + std::marker::Sync + std::marker::Send> IClient for IpfsClient<R> {
    async fn get_id(&self) -> Result<IpfsIdResponse, IpfsClientError> {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/id");
        let response = self.req.post::<IpfsIdResponse, IpfsClientError>(&url).await;

        return response;
    }

    async fn add_file(&self, file_path: &str) -> Result<IpfsAddFileResponse, IpfsClientError> {
        let url = format!("{}{}", self.ipfs_base_url, "/api/v0/add");
        let response = self
            .req
            .post_multipart::<IpfsAddFileResponse, IpfsClientError>(&url, file_path)
            .await;

        return response;
    }

    async fn rm_pin(&self, hash: &str) -> Result<IpfsRemovePinResponse, IpfsClientError> {
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/pin/rm?arg=", hash);
        let response = self
            .req
            .post::<IpfsRemovePinResponse, IpfsClientError>(&url)
            .await;

        return response;
    }

    async fn get(&self, hash: &str) -> Result<IpfsGetResponse, IpfsClientError> {
        let url = format!("{}{}{}", self.ipfs_base_url, "/api/v0/cat?arg=", hash);
        let response = self
            .req
            .post::<IpfsGetResponse, IpfsClientError>(&url)
            .await;

        return response;
    }
}

#[cfg(test)]
pub struct MockIpfsClient {
    expectations: std::collections::HashMap<
        String,
        Box<dyn std::any::Any + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
pub struct Expectation<D: DeserializeOwned> {
    pub func:
        Option<Box<dyn Fn() -> Result<D, IpfsClientError> + std::marker::Sync + std::marker::Send>>,
}

#[cfg(test)]
impl<D: DeserializeOwned + 'static> Expectation<D> {

    pub fn returns(
        &mut self,
        func: impl Fn() -> Result<D, IpfsClientError> + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }
}

#[cfg(test)]
impl MockIpfsClient {
    pub fn new() -> Self {
        return Self {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn _expect_get_id(&mut self) -> &mut Expectation<IpfsIdResponse> {
        self.expectations
            .entry("get_id".to_string())
            .or_insert_with(|| Box::new(Expectation::<IpfsIdResponse> { func: None }))
            .downcast_mut::<Expectation<IpfsIdResponse>>()
            .unwrap()
    }

    pub fn expect_add_file(&mut self) -> &mut Expectation<IpfsAddFileResponse>
    {
        self.expectations
            .entry("add_file".to_string())
            .or_insert_with(|| Box::new(Expectation::<IpfsAddFileResponse> { func: None }))
            .downcast_mut::<Expectation<IpfsAddFileResponse>>()
            .unwrap()
    }

    pub fn expect_rm_pin(&mut self) -> &mut Expectation<IpfsRemovePinResponse>
    {
        self.expectations
            .entry("rm_pin".to_string())
            .or_insert_with(|| Box::new(Expectation::<IpfsRemovePinResponse> { func: None }))
            .downcast_mut::<Expectation<IpfsRemovePinResponse>>()
            .unwrap()
    }

    pub fn expect_get(&mut self) -> &mut Expectation<IpfsGetResponse>
    {
        self.expectations
            .entry("get".to_string())
            .or_insert_with(|| Box::new(Expectation::<IpfsGetResponse> { func: None }))
            .downcast_mut::<Expectation<IpfsGetResponse>>()
            .unwrap()
    }
}

#[cfg(test)]
#[async_trait]
impl IClient for MockIpfsClient {
    async fn get_id(&self) -> Result<IpfsIdResponse, IpfsClientError> {
        let expectation = self
            .expectations
            .get("get_id")
            .unwrap()
            .downcast_ref::<Expectation<IpfsIdResponse>>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    async fn add_file(&self, _file_path: &str) -> Result<IpfsAddFileResponse, IpfsClientError> {
        let expectation = self
            .expectations
            .get("add_file")
            .unwrap()
            .downcast_ref::<Expectation<IpfsAddFileResponse>>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }
    async fn rm_pin(&self, _hash: &str) -> Result<IpfsRemovePinResponse, IpfsClientError> {
        let expectation = self
            .expectations
            .get("rm_pin")
            .unwrap()
            .downcast_ref::<Expectation<IpfsRemovePinResponse>>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    async fn get(&self, _hash: &str) -> Result<IpfsGetResponse, IpfsClientError> {
        let expectation = self
            .expectations
            .get("get")
            .unwrap()
            .downcast_ref::<Expectation<IpfsGetResponse>>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }
}
