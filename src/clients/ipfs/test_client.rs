#[cfg(test)]
mod tests {

    use crate::clients::ipfs::client::{IpfsClient, IClient};
    use crate::clients::ipfs::models::*;
    use crate::clients::reqwest::client::MockReqwestClient;

    const IPFS_ID: &str = "12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn";
    const IPFS_HASH: &str = "QmYNJg8vEZcToKfresVMfCQzXrDnGuYM6TR7EZ8y33bLbE";
    const IPFS_FILE_NAME: &str = "QmYNJg8vEZcToKfresVMfCQzXrDnGuYM6TR7EZ8y33bLbE";

    #[tokio::test]
    async fn test_get_id_should_return_id() {
        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client
            .expect_post::<IpfsIdResponse, IpfsClientError>()
            .returns(|| {
                Ok(IpfsIdResponse {
                    ID: IPFS_ID.to_string(),
                })
            });

        let ipfs_client = IpfsClient {
            req: mock_reqwest_client,
            ipfs_base_url: "".to_string(),
        };
        let ipfs_response = ipfs_client.get_id().await.unwrap();

        assert_eq!(ipfs_response.ID, IPFS_ID.to_string())
    }

    #[tokio::test]
    async fn test_get_id_should_add_file_to_ipfs_and_return_hash() {

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client
            .expect_post_multipart::<IpfsAddFileResponse, IpfsClientError>()
            .returns(|| {
                Ok(IpfsAddFileResponse {
                    Name: IPFS_HASH.to_string(),
                    Hash: IPFS_FILE_NAME.to_string(),
                })
            });

        let ipfs_client = IpfsClient {
            req: mock_reqwest_client,
            ipfs_base_url: "".to_string(),
        };

        let ipfs_add_file_response = ipfs_client.add_file("").await.unwrap();

        assert_eq!(ipfs_add_file_response.Name, IPFS_HASH.to_string());
        assert_eq!(ipfs_add_file_response.Hash, IPFS_FILE_NAME.to_string());
    }

    #[tokio::test]
    async fn test_get_id_should_rm_object_from_pinned_list() {

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client
            .expect_post::<IpfsRemovePinResponse, IpfsClientError>()
            .returns(|| Ok(IpfsRemovePinResponse {
                Pins: vec!["QmRPcXxhQ6tuPeRmei38GZeNsC3kQvxU9Wq65pN8az28Zz".to_string()],
            }));

        let ipfs_client = IpfsClient {
            req: mock_reqwest_client,
            ipfs_base_url: "".to_string(),
        };

        let ipfs_rm_pin_response = ipfs_client.rm_pin("").await.unwrap();

        assert_eq!(ipfs_rm_pin_response.Pins.len(), 1);
    }
}
