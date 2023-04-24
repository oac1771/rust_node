use crate::clients::ipfs::client::IpfsClient;
use crate::clients::reqwest::client::{R, Response};

use mockall::*;
use async_trait::async_trait;

mock! {
        
    pub ReqwestClient{}

    #[async_trait]
    impl R for ReqwestClient {
        async fn post(&self, url: &str) -> Response;
        async fn post_multipart(&self, url: &str, file_name: &str) -> Response;
    }

}


#[tokio::test]
async fn foo() {

    let mut mock_reqwest_client = MockReqwestClient::new();
    mock_reqwest_client.expect_post().times(2);

    let ipfs_client = IpfsClient {
        reqwest_client: Box::new(mock_reqwest_client)
    };

    ipfs_client.get_id().await;

}