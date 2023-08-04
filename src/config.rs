pub struct Config {
    pub ipfs_base_url: String
}

pub fn get_config() -> Config {
    let ipfs_base_url = std::env::var("IPFS_BASE_URL").expect("IPFS_BASE_URL not set");
    return Config {
        ipfs_base_url
    }
}