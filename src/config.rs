use std::str;

use serde_json::json;
use serde::Deserialize;

use ethers::types::Address;

#[derive(Deserialize)]
pub struct Config {
    pub ipfs_config: IpfsConfig,
    pub zksync_config: ZksyncConfig
}


#[derive(Deserialize)]
pub struct IpfsConfig {
    pub ipfs_base_url: String
}

#[derive(Deserialize)]
pub struct ZksyncConfig {
    pub contract_address: Address,
    pub private_key: String,
    pub zksync_api_url: String,
    pub zksync_ws_url: String,
}

pub async fn create_config(path: &str) {

    let ipfs_base_url = std::env::var("IPFS_BASE_URL").expect("IPFS_BASE_URL not set");

    let contract_address_string = std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");
    let contract_address: Address = contract_address_string.parse().expect("Invalid contract address");

    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let zksync_api_url = std::env::var("ZKSYNC_API_URL").expect("ZKSYNC_API_URL not set");
    let zksync_ws_url = std::env::var("ZKSYNC_WS_URL").expect("ZKSYNC_WS_URL not set");

    let config_json = json!({
        "ipfs_config": {
            "ipfs_base_url": ipfs_base_url
        },
        "zksync_config": {
            "contract_address": contract_address,
            "private_key": private_key,
            "zksync_api_url": zksync_api_url,
            "zksync_ws_url": zksync_ws_url
        }
    });

    let config = serde_json::to_string(&config_json).unwrap();
    let parent_directories = path.split("config.json").collect::<Vec<&str>>();

    println!("Config: {:?}", config);
    tokio::fs::create_dir_all(parent_directories[0]).await.unwrap();
    tokio::fs::write(path, config).await.unwrap();

}

pub async fn read_config(path: &str) -> Config {

    let config_bytes = tokio::fs::read(path).await.unwrap();
    let config_string = str::from_utf8(&config_bytes).unwrap();

    let config = serde_json::from_str::<Config>(config_string).unwrap();

    return config
}