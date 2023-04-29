use super::*;
use crate::clients::ipfs::client::IpfsClient;
use mockall_double::double;

mod reqwest_client {
    use mockall::automock;
    use crate::clients::reqwest::models::Response;

    pub struct ReqwestClient{}

    #[automock]
    impl ReqwestClient{
        pub async fn post(&self, url: &str) -> Response {
            return Response{
                status_code: "".to_string(),
                body: "".to_string()
            };
        }
    }
}

#[double]
use reqwest_client::ReqwestClient;


#[tokio::test]
async fn foo() {

    let mut mock_reqwest_client = ReqwestClient::default();
    // mock_reqwest_client.expect_post().times(2);

    // let ipfs_client = IpfsClient {
    //     reqwest_client: mock_reqwest_client
    // };

    // ipfs_client.get_id().await;

}