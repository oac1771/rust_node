use ethers::types::Address;

pub struct Config {
    pub ipfs_config: IpfsConfig,
    pub zksync_config: ZksyncConfig
}

pub struct IpfsConfig {
    pub ipfs_base_url: String
}

pub struct ZksyncConfig {
    pub contract_address: Address,
    pub private_key: String,
    pub zksync_api_url: String,
    pub zksync_ws_url: String,
}

pub fn get_config() -> Config {
    let ipfs_base_url = std::env::var("IPFS_BASE_URL").expect("IPFS_BASE_URL not set");

    let contract_address_string = std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let zksync_api_url = std::env::var("ZKSYNC_API_URL").expect("ZKSYNC_API_URL not set");
    let zksync_ws_url = std::env::var("ZKSYNC_WS_URL").expect("ZKSYNC_WS_URL not set");

    let contract_address: Address = contract_address_string.parse().expect("Invalid contract address");

    let ipfs_config = IpfsConfig{
        ipfs_base_url
    };

    let zksync_config = ZksyncConfig {
        contract_address,
        private_key,
        zksync_api_url,
        zksync_ws_url
    };

    return Config {
        ipfs_config,
        zksync_config
    }
}