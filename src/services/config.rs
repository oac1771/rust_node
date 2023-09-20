use std::{
    str,
    thread,
    time, 
    io::{Read, Write},
    fs
};
use fs2::FileExt;

use serde_json::json;
use serde::{Deserialize, Serialize};

use ethers::types::{Address, H160};

pub const CONFIG_PATH: &str = "./var/config.json";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub ipfs_config: IpfsConfig,
    pub zksync_config: ZksyncConfig
}

#[derive(Deserialize, Serialize)]
pub struct IpfsConfig {
    pub ipfs_base_url: String
}

#[derive(Deserialize, Serialize)]
pub struct ZksyncConfig {
    pub contract_address: Address,
    pub private_key: String,
    pub zksync_api_url: String,
    pub zksync_ws_url: String,
}

pub async fn create_config() {

    let ipfs_base_url = std::env::var("IPFS_BASE_URL").expect("IPFS_BASE_URL not set");

    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let zksync_api_url = std::env::var("ZKSYNC_API_URL").expect("ZKSYNC_API_URL not set");
    let zksync_ws_url = std::env::var("ZKSYNC_WS_URL").expect("ZKSYNC_WS_URL not set");

    let config = json!({
        "ipfs_config": {
            "ipfs_base_url": ipfs_base_url
        },
        "zksync_config": {
            "contract_address": H160::zero(),
            "private_key": private_key,
            "zksync_api_url": zksync_api_url,
            "zksync_ws_url": zksync_ws_url
        }
    });

    // let parent_directories = CONFIG_PATH.split("config.json").collect::<Vec<&str>>();
    // fs::create_dir_all(parent_directories[0]).unwrap();

    write_config(config).await;

}

pub async fn set_contract_address(address: &str) {
    let contract_address: Address = address.parse().expect("Invalid contract address");

    let mut config = read_config().await;
    config.zksync_config.contract_address = contract_address;

    write_config(config).await;
    
}

pub async fn read_config() -> Config {

    println!("reading config");
    // let mut file = tokio::fs::File::open(CONFIG_PATH).await.unwrap().into_std().await;
    let mut file = fs::File::open(CONFIG_PATH).unwrap();

    while let Err(err) = file.try_lock_exclusive() {
        println!("File is locked while trying to read: {:?}", err);
        thread::sleep(time::Duration::from_secs(0.5 as u64));
    }
    let mut contents = String::new();
    let config_bytes = file.read_to_string(&mut contents).unwrap();
    // let config_string = str::from_utf8(&config_bytes).unwrap();
    println!("contents: {:?}", contents);

    let config = serde_json::from_str::<Config>(&contents).unwrap();

    file.unlock();

    return config

}

pub async fn write_config(config: impl Serialize) {

    let config = serde_json::to_string(&config).unwrap();
    let mut file = fs::File::create(CONFIG_PATH).unwrap();

    while let Err(err) = file.try_lock_exclusive() {
        println!("File is locked while trying to write: {:?}", err);
        thread::sleep(time::Duration::from_secs(0.5 as u64));
    }

    file.write_all(config.as_bytes());
    file.unlock();
    println!("config written");

    // tokio::fs::write(CONFIG_PATH, config).await.unwrap();
}