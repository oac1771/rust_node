use ethers::{
    contract::Contract,
    providers::{Provider, Http},
    signers::Wallet,
    types::Address,
};

use crate::config::ZksyncConfig;

// save provider instance as stuct object so it can be mocked
// should use ethers-mock to mock provider

pub struct ZksyncClient {}

impl ZksyncClient {

    pub fn new(config: &ZksyncConfig) -> ZksyncClient {
        let client = Provider::<Http>::try_from(&config.zksync_url).unwrap();
        let contract_address: Address = config.contract_address.parse().expect("Invalid contract address");
        let wallet = Wallet::from_bytes(&config.private_key.as_bytes())
            .expect("Unable to generate Wallet instance");

        return ZksyncClient{}
    }
}