#[cfg(test)]
mod tests {

    use std::{
        any::Any,
        marker::Send,
        path::Path,
        env,
        fs,
        thread,
        time
    };

    use fs2::FileExt;
    use futures::FutureExt;
    use ethers::types::{H160, Address};

    use super::super::config::*;

    async fn run_test<'a>(result: Result<(), Box<dyn Any + Send>>, teardown: impl FnOnce() -> ()) 
    {
        teardown();
        assert!(result.is_ok());
    }

    fn set_env_var() {
        env::set_var("IPFS_BASE_URL", "foo");
        env::set_var("PRIVATE_KEY", "foo");
        env::set_var("ZKSYNC_API_URL", "foo");
        env::set_var("ZKSYNC_WS_URL", "foo");
    }

    fn delete_config_file<'a>() -> impl FnOnce() -> () {
        let teardown = || {

            let file = fs::File::open(CONFIG_PATH);

            match file {
                Ok(file) => {
                    while let Err(err) = file.try_lock_exclusive() {
                        println!("File is locked while trying to cleanup: {:?}", err);
                        thread::sleep(time::Duration::from_secs(0.5 as u64));
                    }
                    let _ = fs::remove_file(CONFIG_PATH);
                    file.unlock();
                },
                Err(_) => {}
            }
        };
        return teardown
    }
    
    #[tokio::test]
    async fn should_create_config_file() {
        let test = async {

            set_env_var();
            create_config().await;

            let path = Path::new(CONFIG_PATH);
            assert_eq!(path.exists(), true);
        };

        let parent_directories = CONFIG_PATH.split("config.json").collect::<Vec<&str>>();
        fs::create_dir_all(parent_directories[0]).unwrap();

        println!("{:?}", parent_directories);

        let paths = fs::read_dir("./").unwrap();
        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }


        assert_eq!(true, false);
        // let result = test.catch_unwind().await;
        // run_test(result, delete_config_file()).await;
    }

    // #[tokio::test]
    // async fn read_config_should_return_struct_with_correct_values() {
    //     let test = async {

    //         set_env_var();
    //         create_config().await;

    //         let config = read_config().await;

    //         assert_eq!(config.ipfs_config.ipfs_base_url, "foo".to_string());
    //         assert_eq!(config.zksync_config.private_key, "foo".to_string());
    //         assert_eq!(config.zksync_config.zksync_api_url, "foo".to_string());
    //         assert_eq!(config.zksync_config.zksync_ws_url, "foo".to_string());
    //         assert_eq!(config.zksync_config.contract_address, H160::zero());

    //     };

    //     let result = test.catch_unwind().await;

    //     run_test(result, delete_config_file()).await;
    // }

    // #[tokio::test]
    // async fn set_contract_address_should_add_address_to_config() {
    //     let test = async {
    //         let address = "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92";
    //         let expected_address: Address = address.parse().expect("Invalid contract address");

    //         set_env_var();
    //         create_config().await;
    //         set_contract_address(address).await;

    //         let config = read_config().await;

    //         assert_eq!(config.zksync_config.contract_address, expected_address);

    //     };

    //     let result = test.catch_unwind().await;

    //     run_test(result, delete_config_file()).await;
    // }
}