#[cfg(test)]
mod tests {

    // add tests for error handling stuff
    
    use crate::clients::ipfs::test_data::data::*;

    use crate::clients::ipfs::client::IpfsClient;
    use crate::clients::reqwest::client::MockReqwestClient;
    use crate::clients::reqwest::models::Response;

    fn create_mocked_response(status_code: String, body: String) -> Response{
        return Response {
            status_code,
            body
        }
    }

    #[tokio::test]
    async fn test_get_id_should_return_id() {
        let body = IPFS_ID_RESPONSE.to_string();
        let response = create_mocked_response("".to_string(), body);

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client.expect_post().return_once(|_| response);

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client
        };

        let ipfs_id_response = ipfs_client.get_id().await;
        assert_eq!(ipfs_id_response.ID, "12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn")

    }

    #[tokio::test]
    async fn test_get_id_should_add_file_to_ipfs_and_return_hash() {
        let body = IPFS_ADD_FILE_RESPONSE.to_string();
        let response = create_mocked_response("".to_string(), body);

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client.expect_post_multipart().return_once(|_, _| response);

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client
        };

        let ipfs_add_file_response = ipfs_client.add_file("").await;
        assert_eq!(ipfs_add_file_response.Name, "QmYNJg8vEZcToKfresVMfCQzXrDnGuYM6TR7EZ8y33bLbE");
        assert_eq!(ipfs_add_file_response.Hash, "QmYNJg8vEZcToKfresVMfCQzXrDnGuYM6TR7EZ8y33bLbE");

    }

    #[tokio::test]
    async fn test_get_id_should_rm_object_from_pinned_list() {
        let body = IPFS_REMOVE_PIN_RESPONSE.to_string();
        let response = create_mocked_response("".to_string(), body);

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client.expect_post().return_once(|_| response);

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client
        };

        let ipfs_rm_pin_response = ipfs_client.rm_pin("").await;
        assert_eq!(ipfs_rm_pin_response.Pins, vec!["QmRPcXxhQ6tuPeRmei38GZeNsC3kQvxU9Wq65pN8az28Zz"]);

    }

    
}