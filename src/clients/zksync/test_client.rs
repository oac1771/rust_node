#[cfg(test)]
mod tests {

    use crate::clients::zksync::{
        client::ZksyncClient, contracts::identifier_trait::MockIdentifier, models::Registration,
    };
    use ethers::{
        providers::{MockProvider, Provider},
        types::{Address, H256, U256},
    };

    fn get_contract_address() -> Address {
        return "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"
            .parse()
            .unwrap();
    }

    #[tokio::test]
    async fn test_register_identity_should_return_tx_hash() {
        let mut mock_contract = MockIdentifier::new();

        mock_contract.expect_register_identity().returns(|| {
            let tx_hash = H256::zero();
            return tx_hash;
        });

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
            http_provider: Provider::new(MockProvider::new())
        };

        let hash = client
            .register_identity(
                &"0x36615Cf349d7F6344891B1e7CA7C72883F5dc049",
                &"ipfs_address".to_string(),
                &"data_hash".to_string(),
            )
            .await;
        assert_eq!(hash, H256::zero())
    }

    // #[tokio::test]
    // async fn test_remove_identity_should_return_tx_hash() {
    //     let mut mock_contract = MockIdentifier::new();

    //     mock_contract.expect_remove_identity().returns(|| {
    //         let tx_hash = H256::zero();
    //         return tx_hash;
    //     });

    //     let client = ZksyncClient {
    //         api_url: "".to_string(),
    //         contract: mock_contract,
    //         http_provider: Provider::new(MockProvider::new())
    //     };

    //     let hash = client
    //         .remove_identity(&"0x36615Cf349d7F6344891B1e7CA7C72883F5dc049", 0)
    //         .await;
    //     assert_eq!(hash, H256::zero())
    // }

    // #[tokio::test]
    // async fn test_get_token_id_should_return_token() {
    //     let principal = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049";
    //     let contract_address = get_contract_address();
    //     let mut mock_contract = MockIdentifier::new();

    //     mock_contract.expect_decode().returns(|| {
    //         let event = Registration {
    //             token_id: U256::zero(),
    //             principal: principal.parse().unwrap(),
    //         };
    //         return Ok(event);
    //     });
    //     mock_contract
    //         .expect_get_address()
    //         .returns_const(contract_address);

    //     let client = ZksyncClient {
    //         api_url: "".to_string(),
    //         contract: mock_contract,
    //         http_provider: Provider::new(MockProvider::new())
    //     };

    //     let id = client.get_token_id(principal).await;
    // }
}
