#[cfg(test)]
mod tests {

    use std::{
        any::Any,
        marker::Send,
        path::Path,
        env,
        future::Future
    };
    use futures::FutureExt;
    use ethers::types::{H160, Address};

    use super::super::config::*;

    fn run_test(result: Result<(), Box<dyn Any + Send>>, teardown: impl FnOnce() -> dyn Future<Output = ()>) {
        teardown();
        assert!(result.is_ok());
    }

    fn set_env_var() {
        env::set_var("IPFS_BASE_URL", "foo");
        env::set_var("PRIVATE_KEY", "foo");
        env::set_var("ZKSYNC_API_URL", "foo");
        env::set_var("ZKSYNC_WS_URL", "foo");
    }

    async fn delete_config_file() {
        _ = tokio::fs::remove_file(CONFIG_PATH).await;
    }
    
    #[tokio::test]
    async fn should_create_config_file() {
        let test = async {

            set_env_var();
            create_config().await;

            let path = Path::new(CONFIG_PATH);
            assert_eq!(path.exists(), true);
        };

        let result = test.catch_unwind().await;

        run_test(result, delete_config_file);
    }

    #[tokio::test]
    async fn read_config_should_return_struct_with_correct_values() {
        let test = async {

            set_env_var();
            create_config().await;

            let config = read_config().await;

            assert_eq!(config.ipfs_config.ipfs_base_url, "foo".to_string());
            assert_eq!(config.zksync_config.private_key, "foo".to_string());
            assert_eq!(config.zksync_config.zksync_api_url, "foo".to_string());
            assert_eq!(config.zksync_config.zksync_ws_url, "foo".to_string());
            assert_eq!(config.zksync_config.contract_address, H160::zero());

        };

        let result = test.catch_unwind().await;

        run_test(result, delete_config_file);
    }

    #[tokio::test]
    async fn set_contract_address_should_add_address_to_config() {
        let test = async {
            let address = "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92";
            let expected_address: Address = address.parse().expect("Invalid contract address");

            set_env_var();
            create_config().await;
            set_contract_address(address).await;

            let config = read_config().await;

            assert_eq!(config.zksync_config.contract_address, expected_address);

        };

        let result = test.catch_unwind().await;

        run_test(result, delete_config_file);
    }
}