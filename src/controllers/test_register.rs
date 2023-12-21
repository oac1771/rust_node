#[cfg(test)]
mod tests {

    use crate::clients::{
        ipfs::{client::MockIpfsClient, models::IpfsAddFileResponse},
        zksync::client::MockZksyncClient,
    };

    use crate::services::{
        identity::MockIdentityService, models::Data, models::Identity, state::MockStateService,
    };

    use crate::controllers::register::RegisterController;

    use ethers::{
        abi::Token,
        types::{H256, U256},
    };
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_authenticate_returns_ok_enum() {
        let mut mock_ipfs_client = MockIpfsClient::new();
        let mock_state_service = MockStateService::new();
        let mut mock_zksync_client = MockZksyncClient::new();
        let mut mock_identity_service = MockIdentityService::new();

        let principal_address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049";
        let tx_hash = H256::zero();
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

        mock_ipfs_client.expec_add_file().returns(|| {
            let response = IpfsAddFileResponse {
                Name: "name".to_string(),
                Hash: ipfs_address.to_string(),
            };
            return Ok(response);
        });

        mock_zksync_client.expect_register_identity().returns(move || {
            return tx_hash;
        });
        mock_zksync_client
            .expect_get_token_id()
            .returns_token(|| return Some(Token::Uint(U256::zero())));

        let register_controller = RegisterController {
            ipfs_client: mock_ipfs_client,
            state_service: mock_state_service,
            identity_service: mock_identity_service,
            zksync_client: mock_zksync_client,
            check_identity: false,
        };

        let response = register_controller.register(data, principal_address).await;

        let expected_response = serde_json::json!({
            "tx_hash": tx_hash,
            "token_id": Token::Uint(U256::zero()).to_string(),
            "ipfs_address": ipfs_address,
            "encryption_key": encryption_key
        });

        assert_eq!(response.error, None);
        assert_eq!(response.body, Some(expected_response));
    }
}
