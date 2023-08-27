use ethers::{
    providers::{Provider, Http, Ws, Middleware},
    middleware::SignerMiddleware,
    types::{U256, Address},
    signers::{LocalWallet, Signer},
};

use std::{convert::TryFrom, sync::Arc};

use crate::config::ZksyncConfig;
use crate::identifier::Identifier;

pub struct ZksyncClient {
    pub contract: Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl ZksyncClient {

    pub async fn new(config: &ZksyncConfig) -> ZksyncClient {

        let http_provider = Provider::<Http>::try_from(&config.zksync_api_url).unwrap();
        let chain_id = http_provider.get_chainid().await.unwrap().as_u64();
    
        let wallet = config.private_key.parse::<LocalWallet>().unwrap().with_chain_id(chain_id);
        let client = SignerMiddleware::new(http_provider, wallet);
    
        let contract = Identifier::new(config.contract_address, Arc::new(client));

        return ZksyncClient{
            contract
        }
    }

    pub async fn get_current_token_id(&self) -> U256 {
        let token_id = self.contract.get_current_token_id().call().await.unwrap();
        return token_id
    }

    pub async fn register_identity(&self, principal: Address, ipfs_address: String, data_hash: String) {

        let call = self.contract.register_identity(principal, ipfs_address, data_hash);
        let tx = call.send().await.unwrap().await.unwrap();
        println!("transaction: {:?}", tx.unwrap().transaction_hash);

    }

}