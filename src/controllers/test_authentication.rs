#[cfg(test)]
mod tests {

    use crate::clients::{
        ipfs::{client::MockIpfsClient, models::IpfsGetResponse},
        zksync::contracts::identifier::AuthenticationRequestFilter,
    };

    use crate::services::{
        identity::MockIdentityService, models::Identity, state::MockStateService,
    };

    use crate::controllers::authentication::AuthenticationController;

    use ethers::types::Address;

    #[tokio::test]
    async fn test_authenticate_returns_ok_enum() {
        let mut mock_ipfs_client = MockIpfsClient::new();
        let mut mock_state_service = MockStateService::new();
        let mut mock_identity_service = MockIdentityService::new();

        let principal: Address = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049".parse().unwrap();

        let request = AuthenticationRequestFilter {
            principal,
            ipfs_address: "ipfs_address".to_string(),
            data_hash: "hash".to_string(),
        };

        mock_ipfs_client.expect_get().returns(|| {
            let ipfs_response = IpfsGetResponse {
                data: "ipfs_data".to_string(),
            };
            return Ok(ipfs_response);
        });

        mock_state_service
            .expect_get_encryption_key()
            .returns(|| return Some("encryption_key".to_string()));

        mock_identity_service
            .expect_regenerate_identity()
            .returns(|| {
                let identity = Identity {
                    hash: "hash".to_string(),
                    encryption_key: "encryption_key".to_string(),
                    data: "data".to_string(),
                };
                return Ok(identity);
            });

        let authentication_controller = AuthenticationController {
            ipfs_client: mock_ipfs_client,
            state_service: mock_state_service,
            identity_service: mock_identity_service,
        };

        let response = authentication_controller.authenticate(request).await.unwrap();

        assert_eq!(response, ())
    }
}
