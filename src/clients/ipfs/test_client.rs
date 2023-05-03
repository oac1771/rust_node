#[cfg(test)]
mod tests {
    use crate::clients::ipfs::client::IpfsClient;
    use crate::clients::reqwest::client::MockReqwestClient;

    #[tokio::test]
    async fn foo() {

        let mut mock_reqwest_client = MockReqwestClient::default();

        mock_reqwest_client.expect_post().times(2);

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client
        };

        ipfs_client.get_id().await;

    }

    
}