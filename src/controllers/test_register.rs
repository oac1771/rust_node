#[cfg(test)]
mod tests {

    use crate::clients::{
        ipfs::{
            client::MockIpfsClient,
            models::{IpfsAddFileResponse, IpfsRemovePinResponse},
        },
        zksync::client::MockZksyncClient,
    };

    use crate::services::{
        identity::MockIdentityService, models::Data, models::Identity, state::MockStateService,
    };

    use crate::controllers::register::RegisterController;

    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_register_returns_response_without_error() {
        let mut mock_ipfs_client = MockIpfsClient::new();
        let mock_state_service = MockStateService::new();
        let mut mock_zksync_client = MockZksyncClient::new();
        let mut mock_identity_service = MockIdentityService::new();

        let principal_address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049";
        let tx_hash = "0x00000";
        let token_id = 0;
        let ipfs_address = "ipfs_address";
        let encryption_key = "encryption_key";

        let data = Data {
            meta_data: "meta_data".to_string(),
            data: serde_json::json!({}),
        };
        mock_identity_service.expect_create_identity().returns(|| {
            let temp_file = NamedTempFile::new()?;
            let identity = Identity {
                hash: "hash".to_string(),
                encryption_key: encryption_key.to_string(),
                data: "data".to_string(),
            };
            return Ok((temp_file, identity));
        });

        mock_ipfs_client.expect_add_file().returns(|| {
            let response = IpfsAddFileResponse {
                Name: "name".to_string(),
                Hash: ipfs_address.to_string(),
            };
            return Ok(response);
        });

        mock_zksync_client
            .expect_register_identity()
            .returns_string(move || {
                return tx_hash.to_string();
            });
        mock_zksync_client
            .expect_get_token_id()
            .returns_u64(move || return Some(token_id));

        let register_controller = RegisterController {
            ipfs_client: mock_ipfs_client,
            state_service: Box::new(mock_state_service),
            identity_service: Box::new(mock_identity_service),
            zksync_client: mock_zksync_client,
            check_identity: false,
        };

        let response = register_controller
            .register(data, principal_address)
            .await
            .unwrap();

        assert_eq!(response.encryption_key, encryption_key);
        assert_eq!(response.ipfs_address, ipfs_address);
        assert_eq!(response.token_id, token_id);
        assert_eq!(response.tx_hash, tx_hash.clone());
    }

    #[tokio::test]
    async fn test_remove_returns_response_without_error() {
        let mut mock_ipfs_client = MockIpfsClient::new();
        let mock_state_service = MockStateService::new();
        let mut mock_zksync_client = MockZksyncClient::new();
        let mock_identity_service = MockIdentityService::new();

        let principal_address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049";
        let token_id = 0;
        let tx_hash = "0x00000";

        mock_zksync_client
            .expect_remove_identity()
            .returns_string(move || {
                return tx_hash.to_string();
            });
        mock_zksync_client
            .expect_get_ipfs_addr()
            .returns_string(|| return Some(String::new()));

        mock_ipfs_client.expect_rm_pin().returns(|| {
            let pins = vec![];
            let response = IpfsRemovePinResponse { Pins: pins };
            return Ok(response);
        });

        let register_controller = RegisterController {
            ipfs_client: mock_ipfs_client,
            state_service: Box::new(mock_state_service),
            identity_service: Box::new(mock_identity_service),
            zksync_client: mock_zksync_client,
            check_identity: false,
        };

        let response = register_controller
            .remove(principal_address, token_id)
            .await
            .unwrap();

        assert_eq!(response.tx_hash, tx_hash);
        assert_eq!(response.removed_pins, Vec::<String>::new());
    }
}
