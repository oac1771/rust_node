use std::str;

use ethers::{
    providers::{Provider, Http, Middleware},
    middleware::SignerMiddleware,
    types::{U256, Address, H256},
    signers::{LocalWallet, Signer},
    prelude::FunctionCall, 
    abi::Detokenize
};

use std::{convert::TryFrom, sync::Arc};

use crate::services::config::ZksyncConfig;
use crate::identifier::{Identifier, IdentifierEvents};

pub struct ZksyncClient {
    pub contract: Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

// change queries to use manual log filtering ones so you dont have to check all events, only specific ones you care about

// make get_token_id use same base query() method that does error handling
// this wont really matter until error handling on awaiting transactions are done

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
            ipfs_address.to_string(), 
            data_hash.to_string());

        let tx_hash = self.send(call).await;

        return tx_hash

    }

    pub async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256 {
        let principal: Address = principal_address.parse().expect("Invalid principal address");
        let token: U256 = U256::from(token_id);

        let call = self.contract.remove_identity(token, principal);
        let tx_hash = self.send(call).await;

        return tx_hash

    }

    pub async fn check_identity(&self, principal_address: &str) -> bool {
        let principal: Address = principal_address.parse().expect("Invalid principal address");

        let call = self.contract.check_identity(principal);
        let identity_status = self.call::<bool>(call).await;

        return identity_status
    }

    pub async fn get_current_token_id(&self) -> U256 {
        let call = self.contract.get_current_token_id();
        let token_id = self.call::<U256>(call).await; 

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

    pub async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<String>{
        let principal: Address = principal_address.parse().expect("Invalid principal address");
        let token: U256 = U256::from(token_id);

        let events = self.contract.events().from_block(0).query().await.unwrap();

        for event in events {
            match event {
                IdentifierEvents::IpfsDeletionRequestFilter(request) => {
                    if request.principal == principal && request.token_id == token {
                        return Some(request.ipfs_address)
                    }
                },
                _ => {}
            }
        }
        return None
    }

    async fn send(&self, call: FunctionCall<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, 
        SignerMiddleware<Provider<Http>, 
        LocalWallet>, ()>) -> H256 {
        
        let tx = call.send().await.unwrap().await.unwrap();
        let tx_hash = tx.unwrap().transaction_hash;

        return tx_hash

    }

    async fn call<T>(&self, call: FunctionCall<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, 
        SignerMiddleware<Provider<Http>, 
        LocalWallet>, T>) -> T 
    where T: Detokenize    
    {
        
        let result: T = call.call().await.unwrap();
        return result

    }

}