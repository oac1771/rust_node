use crate::clients::ipfs::client::IpfsClient;
use crate::clients::reqwest::client::{R, Response};

use mockall::*;
use async_trait::async_trait;

#[test]
fn foo() {

    let ipfs_client = create_stubbed_client();
    ipfs_client.reqwest_client
}


fn create_stubbed_client() -> IpfsClient{
    mock! {
        
        pub ReqwestClient{}
    
        #[async_trait]
        impl R for ReqwestClient {
            async fn post(&self, url: &str) -> Response;
            async fn post_multipart(&self, url: &str, file_name: &str) -> Response;
        }

    }

    let mut mock_reqwest_client = MockReqwestClient::new();
    let ipfs_client = IpfsClient{
        reqwest_client: mock_reqwest_client
    };

    return ipfs_client
}