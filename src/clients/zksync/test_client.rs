#[cfg(test)]
mod tests {

    use crate::clients::zksync::{
        client::ZksyncClient, contracts::identifier_trait::MockIdentifier,
    };
    use ethers::types::{Address, H256};

    const PRIVATE_KEY: &str = "0xac1e735be8536c6534bb4f17f06f6afc73b2b5ba84ac2cfb12f7461b20c0bbe3";

    fn get_contract_address() -> Address {
        return "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"
            .parse()
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_id_should_return_id() {
        let mut mock_contract = MockIdentifier::new();

        mock_contract.expect_register_identity().returns(|| {
            let tx_hash = H256::zero();
            return tx_hash;
        });

        let client = ZksyncClient {
            api_url: "".to_string(),
            contract: mock_contract,
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
}
