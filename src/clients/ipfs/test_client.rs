use crate::clients::ipfs::client::IpfsClient;
use crate::clients::reqwest::client::*;
use mockall::*;

#[test]
fn foo() {

}

// probably have to make reqwest client impl a trait and have ipfs client take in that trait

fn create_stubbed_client() {
    mock! {

        ReqwestClient{}

        impl R for ReqwestClient {
            fn new() -> ReqwestClient;
            async fn post(&self)
        }
    }
}