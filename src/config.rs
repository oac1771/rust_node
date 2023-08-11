use std::collections::HashMap;
use std::sync::Mutex;

pub struct Config {
    pub ipfs_config: IpfsConfig,
    pub encryption_config: EncryptionConfig,
    pub zksync_config: ZksyncConfig
}

pub struct IpfsConfig {
    pub ipfs_base_url: String
}

pub struct EncryptionConfig {
    pub private_keys: Mutex<HashMap<String, String>>
}

pub struct ZksyncConfig {
    pub contract_address: String,
    pub private_key: String,
    pub zksync_url: String
}

pub fn get_config() -> Config {
    let ipfs_base_url = std::env::var("IPFS_BASE_URL").expect("IPFS_BASE_URL not set");

    let contract_address = std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let zksync_url = std::env::var("ZKSYNC_URL").expect("ZKSYNC_URL not set");


    let ipfs_config = IpfsConfig{
        ipfs_base_url
    };

    let encryption_config = EncryptionConfig{
        private_keys: Mutex::new(HashMap::new())
    };

    let zksync_config = ZksyncConfig{
        contract_address,
        private_key,
        zksync_url
    };

    return Config {
        ipfs_config,
        encryption_config,
        zksync_config
    }
}