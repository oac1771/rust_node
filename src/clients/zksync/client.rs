use std::str;

use ethers::{
    providers::{Provider, Http, Middleware},
    middleware::SignerMiddleware,
    types::{U256, Address, H256, Bytes},
    signers::{LocalWallet, Signer},
};

use std::{convert::TryFrom, sync::Arc};

use crate::config::ZksyncConfig;
use crate::identifier::{Identifier, IdentifierEvents};

pub struct ZksyncClient {
    pub contract: Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

// make register_identity and remove_identity use same base send() method that does error handling
    // just pass in self.contract.<method>

// make check_identity, get_current_token use same base call() method that does error handling
// make get_token_id use same base query() method that does error handling

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

    pub async fn register_identity(&self, principal_address: &str, ipfs_address: &str, data_hash: &str) -> H256 {
        let principal: Address = principal_address.parse().expect("Invalid principal address");

        let call = self.contract.register_identity(principal, 
            self.to_bytes(ipfs_address), 
            self.to_bytes(data_hash));
        let tx = call.send().await.unwrap().await.unwrap();
        let tx_hash = tx.unwrap().transaction_hash;

        return tx_hash

    }

    pub async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256 {
        let principal: Address = principal_address.parse().expect("Invalid principal address");
        let token: U256 = U256::from(token_id);

        let call = self.contract.remove_identity(token, principal);
        let tx = call.send().await.unwrap().await.unwrap();

        let tx_hash = tx.unwrap().transaction_hash;

        return tx_hash

    }

    pub async fn check_identity(&self, principal_address: &str) -> bool {
        let principal: Address = principal_address.parse().expect("Invalid principal address");

        let call = self.contract.check_identity(principal).call().await.unwrap();
        return call
    }

    pub async fn get_current_token_id(&self) -> U256 {
        let token_id = self.contract.get_current_token_id().call().await.unwrap();
        return token_id
    }

    pub async fn get_token_id(&self, principal_address: &str) -> Option<U256> {
        let principal: Address = principal_address.parse().expect("Invalid principal address");
        let events = self.contract.events().from_block(0).query().await.unwrap();

        for event in events {
            match event {
                IdentifierEvents::TransferFilter(transfer) => {
                    if transfer.to == principal {
                        return Some(transfer.token_id)
                    }
                },
                _ => {}
            }
        }
        return None
    }

    pub async fn get_ipfs_addr(&self, principal_address: &str) -> Option<String>{
        let principal: Address = principal_address.parse().expect("Invalid principal address");
        let events = self.contract.events().from_block(0).query().await.unwrap();

        for event in events {
            match event {
                IdentifierEvents::IpfsDeletionRequestFilter(request) => {
                    if request.principal == principal {
                        let ipfs_address = self.from_bytes(request.ipfs_address);
                        return Some(ipfs_address)
                    }
                },
                _ => {}
            }
        }
        return None
    }

    pub fn to_bytes(&self, str: &str) -> Bytes {
        let bytes = Bytes(str.as_bytes().to_vec().into());
        return bytes
    }

    pub fn from_bytes(&self, bytes: Bytes) -> String {
        return str::from_utf8(&bytes).unwrap().to_string()
    }

}