use crate::config::ZksyncConfig;

// save provider instance as stuct object so it can be mocked
// should use ethers-mock to mock provider

pub struct ZksyncClient {}

impl ZksyncClient {

    pub fn new(_config: &ZksyncConfig) -> ZksyncClient {

        return ZksyncClient{}
    }
}