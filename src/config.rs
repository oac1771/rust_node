use std::collections::HashMap;
use std::sync::Mutex;

pub struct Config {
    pub ipfs_config: IpfsConfig,
    pub encryption_config: EncryptionConfig
}

pub struct IpfsConfig {
    pub ipfs_base_url: String
}

pub struct EncryptionConfig {
    pub private_keys: Mutex<HashMap<String, String>>
}

pub fn get_config() -> Config {
    let ipfs_base_url = std::env::var("IPFS_BASE_URL").expect("IPFS_BASE_URL not set");

    let ipfs_config = IpfsConfig{
        ipfs_base_url
    };

    let encryption_config = EncryptionConfig{
        private_keys: Mutex::new(HashMap::new())
    };

    return Config {
        ipfs_config,
        encryption_config
    }
}