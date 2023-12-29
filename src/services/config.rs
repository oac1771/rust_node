use std::str;

use serde::{Deserialize, Serialize};
use serde_json::json;

use ethers::types::{Address, H160};

use super::models::ConfigServiceError;

pub const CONFIG_PATH: &str = "./var/config.json";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub ipfs_config: IpfsConfig,
    pub zksync_config: ZksyncConfig,
    pub check_identity: bool,
}

#[derive(Deserialize, Serialize)]
pub struct IpfsConfig {
    pub ipfs_base_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct ZksyncConfig {
    pub contract_address: Address,
    pub private_key: String,
    pub zksync_api_url: String,
    pub zksync_ws_url: String,
}

pub async fn create_config() -> Result<(), ConfigServiceError> {
    let check_identity = std::env::var("CHECK_ID")?.parse::<bool>()?;

    let ipfs_base_url = std::env::var("IPFS_BASE_URL")?;

    let private_key = std::env::var("PRIVATE_KEY")?;
    let zksync_api_url = std::env::var("ZKSYNC_API_URL")?;
    let zksync_ws_url = std::env::var("ZKSYNC_WS_URL")?;

    let config = json!({
        "ipfs_config": {
            "ipfs_base_url": ipfs_base_url
        },
        "zksync_config": {
            "contract_address": H160::zero(),
            "private_key": private_key,
            "zksync_api_url": zksync_api_url,
            "zksync_ws_url": zksync_ws_url
        },
        "check_identity": check_identity
    });

    let parent_directories = CONFIG_PATH.split("config.json").collect::<Vec<&str>>();
    tokio::fs::create_dir_all(parent_directories[0]).await?;

    write_config(config).await?;

    return Ok(());
}

pub async fn set_contract_address(address: &str) -> Result<(), ConfigServiceError> {
    let contract_address: Address = address.parse()?;

    let mut config = read_config().await?;
    config.zksync_config.contract_address = contract_address;

    write_config(config).await?;
    return Ok(());
}

pub async fn read_config() -> Result<Config, ConfigServiceError> {
    let config_bytes = tokio::fs::read(CONFIG_PATH).await?;
    let config_string = str::from_utf8(&config_bytes)?;

    let config = serde_json::from_str::<Config>(config_string)?;

    return Ok(config);
}

pub async fn write_config(config: impl Serialize) -> Result<(), ConfigServiceError> {
    let config = serde_json::to_string(&config)?;
    tokio::fs::write(CONFIG_PATH, config).await?;

    return Ok(());
}
