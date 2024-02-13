#[cfg(test)]
mod tests {

    use crate::clients::zksync::{
        client::{ZksyncClient, ZClient},
        contracts::ethers_traits::{MockHttpProvider, MockIdentifier},
        models::{IpfsDeletionRequest, Registration},
    };
    use ethers::types::{Address, Bytes, Log, H256, U256};

    fn get_contract_address() -> Address {
        return "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"
            .parse()
            .unwrap();
    }

    #[tokio::test]
    async fn test_register_identity_should_return_tx_hash() {
        let mut mock_contract = MockIdentifier::new();
        let mock_http_provider = MockHttpProvider::new();

        mock_contract.expect_register_identity().returns(|| {
            let tx_hash = H256::zero();
            return Ok(tx_hash);
        });

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
            http_provider: Box::new(mock_http_provider),
        };

        let hash = client
            .register_identity(
                &"0x36615Cf349d7F6344891B1e7CA7C72883F5dc049",
                &"ipfs_address".to_string(),
                &"data_hash".to_string(),
            )
            .await;
        assert_eq!(hash.unwrap(), H256::zero().to_string())
    }

    #[tokio::test]
    async fn test_remove_identity_should_return_tx_hash() {
        let mut mock_contract = MockIdentifier::new();
        let mock_http_proiver = MockHttpProvider::new();

        mock_contract.expect_remove_identity().returns(|| {
            let tx_hash = H256::zero();
            return Ok(tx_hash);
        });

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
            http_provider: Box::new(mock_http_proiver),
        };

        let hash = client
            .remove_identity(&"0x36615Cf349d7F6344891B1e7CA7C72883F5dc049", 0)
            .await;
        assert_eq!(hash.unwrap(), H256::zero().to_string())
    }

    #[tokio::test]
    async fn test_get_token_id_should_return_token() {
        let principal = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049";
        let contract_address = get_contract_address();
        let mut mock_contract = MockIdentifier::new();
        let mut mock_http_provider = MockHttpProvider::new();

        mock_contract.expect_decode().returns(|| {
            let event = Registration {
                token_id: U256::zero(),
                principal: principal.parse().unwrap(),
            };
            return Ok(event);
        });
        mock_http_provider.expect_logs().returns(|| {
            let mut logs: Vec<Log> = Vec::new();
            let topics: Vec<H256> = Vec::new();
            let log = Log {
                address: Address::random(),
                topics: topics,
                data: Bytes::new(),
                block_hash: None,
                block_number: None,
                transaction_hash: None,
                transaction_index: None,
                log_index: None,
                transaction_log_index: None,
                log_type: None,
                removed: None,
            };
            logs.push(log);
            return Ok(logs);
        });

        mock_contract
            .expect_get_address()
            .returns_const(contract_address);

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
            http_provider: Box::new(mock_http_provider),
        };

        let id = client.get_token_id(principal).await.unwrap().unwrap();

        assert_eq!(id, 0);
    }

    #[tokio::test]
    async fn test_get_ipfs_addr_should_return_token() {
        let principal = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049";
        let contract_address = get_contract_address();
        let token_id = U256::zero();
        let ipfs_addr = "ipfs_addr";

        let mut mock_contract = MockIdentifier::new();
        let mut mock_http_provider = MockHttpProvider::new();

        mock_contract.expect_decode().returns(move || {
            let event = IpfsDeletionRequest {
                token_id: token_id,
                principal: principal.parse().unwrap(),
                ipfs_addr: ipfs_addr.to_string()
            };
            return Ok(event);
        });
        mock_http_provider.expect_logs().returns(|| {
            let mut logs: Vec<Log> = Vec::new();
            let topics: Vec<H256> = Vec::new();
            let log = Log {
                address: Address::random(),
                topics: topics,
                data: Bytes::new(),
                block_hash: None,
                block_number: None,
                transaction_hash: None,
                transaction_index: None,
                log_index: None,
                transaction_log_index: None,
                log_type: None,
                removed: None,
            };
            logs.push(log);
            return Ok(logs);
        });

        mock_contract
            .expect_get_address()
            .returns_const(contract_address);

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
            http_provider: Box::new(mock_http_provider),
        };

        let returned_ipfs_addr = client.get_ipfs_addr(principal, 0).await.unwrap().unwrap();

        assert_eq!(returned_ipfs_addr, ipfs_addr.to_string());
    }

    #[tokio::test]
    async fn test_check_identity_should_return_bool() {
        let mut mock_contract = MockIdentifier::new();
        let mock_http_proiver = MockHttpProvider::new();

        mock_contract.expect_check_identity().returns_bool(|| Ok(true));

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
            http_provider: Box::new(mock_http_proiver),
        };

        let result = client
            .check_identity(&"0x36615Cf349d7F6344891B1e7CA7C72883F5dc049")
            .await;
        assert_eq!(result.unwrap(), true)
    }
}
