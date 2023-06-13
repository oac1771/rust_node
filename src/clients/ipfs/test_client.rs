#[cfg(test)]
mod tests {
    
    use crate::clients::ipfs::test_data::data::*;
    use crate::clients::ipfs::client::IpfsClient;
    use crate::clients::ipfs::models::*;
    use crate::clients::reqwest::client::MockReqwestClient;
    use crate::clients::reqwest::models::{Response, Error};

    fn create_mocked_response(status_code: String, body: String) -> Response {
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
        mock_reqwest_client.expect_post().return_once(|_| Ok(response));

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client,
            ipfs_base_url: "".to_string()
        };

        let ipfs_id_response: IpfsIdResponse = serde_json::from_str(&ipfs_client.get_id().await).unwrap();
        assert_eq!(ipfs_id_response.ID, "12D3KooWPoZPm5khvdtczdCCJYdo3YfPSL43APWL1vQdzMZjM2wn")

    }

    #[tokio::test]
    async fn test_get_id_should_add_file_to_ipfs_and_return_hash() {
        let body = IPFS_ADD_FILE_RESPONSE.to_string();
        let response = create_mocked_response("".to_string(), body);

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client.expect_post_multipart().return_once(|_, _| Ok(response));

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client,
            ipfs_base_url: "".to_string()
        };

        let ipfs_add_file_response: IpfsAddFileResponse = serde_json::from_str(&ipfs_client.add_file("").await).unwrap();
        assert_eq!(ipfs_add_file_response.Name, "QmYNJg8vEZcToKfresVMfCQzXrDnGuYM6TR7EZ8y33bLbE");
        assert_eq!(ipfs_add_file_response.Hash, "QmYNJg8vEZcToKfresVMfCQzXrDnGuYM6TR7EZ8y33bLbE");

    }

    #[tokio::test]
    async fn test_get_id_should_rm_object_from_pinned_list() {
        let body = IPFS_REMOVE_PIN_RESPONSE.to_string();
        let response = create_mocked_response("".to_string(), body);

        let mut mock_reqwest_client = MockReqwestClient::new();
        mock_reqwest_client.expect_post().return_once(|_| Ok(response));

        let ipfs_client = IpfsClient {
            reqwest_client: mock_reqwest_client,
            ipfs_base_url: "".to_string()
        };

        let ipfs_rm_pin_response: IpfsRemovePinResponse = serde_json::from_str(&ipfs_client.rm_pin("").await).unwrap();

        assert_eq!(ipfs_rm_pin_response.Pins, vec!["QmRPcXxhQ6tuPeRmei38GZeNsC3kQvxU9Wq65pN8az28Zz"]);

    }

    #[tokio::test]
    async fn test_handle_should_return_error_body() {
        let error_msg = "Error!";
        let error = Error{body: error_msg.to_string()};

        let mut _mock_reqwest_client = MockReqwestClient::new();
        let ipfs_client = IpfsClient {
            reqwest_client: _mock_reqwest_client,
            ipfs_base_url: "".to_string()
        };

        let handled_error = ipfs_client.handle::<IpfsIdResponse>(&Err(error));

        assert_eq!(handled_error, error_msg);

    }
    
}