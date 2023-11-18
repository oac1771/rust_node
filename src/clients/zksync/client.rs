use std::str;
use std::{convert::TryFrom, sync::Arc};

use ethers::{
    abi::{Detokenize, Token},
    middleware::SignerMiddleware,
    prelude::FunctionCall,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Filter, H256, U256},
};

use super::{
    contracts::identifier::Identifier,
    models::{Event, IpfsDeletionRequest, Registration},
};

use crate::services::config::ZksyncConfig;

pub struct ZksyncClient {
    pub contract: Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub api_url: String,
}

impl ZksyncClient {
    pub async fn new(config: &ZksyncConfig) -> ZksyncClient {
        let http_provider = Provider::<Http>::try_from(&config.zksync_api_url).unwrap();
        let chain_id = http_provider.get_chainid().await.unwrap().as_u64();

        let wallet = config
            .private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id);
        let client = SignerMiddleware::new(http_provider, wallet);

        let contract = Identifier::new(config.contract_address, Arc::new(client));

        return ZksyncClient {
            contract,
            api_url: config.zksync_api_url.to_string(),
        };
    }

    pub async fn register_identity(
        &self,
        principal_address: &str,
        ipfs_address: &str,
        data_hash: &str,
    ) -> H256 {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");

        let call = self.contract.register_identity(
            principal,
            ipfs_address.to_string(),
            data_hash.to_string(),
        );

        let tx_hash = self.send(call).await;

        return tx_hash;
    }

    pub async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256 {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");
        let token: U256 = U256::from(token_id);

        let call = self.contract.remove_identity(token, principal);
        let tx_hash = self.send(call).await;

        return tx_hash;
    }

    pub async fn _check_identity(&self, principal_address: &str) -> bool {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");

        let call = self.contract.check_identity(principal);
        let identity_status = self._call::<bool>(call).await;

        return identity_status;
    }

    pub async fn _get_current_token_id(&self) -> U256 {
        let call = self.contract.get_current_token_id();
        let token_id = self._call::<U256>(call).await;

        return token_id;
    }

    pub async fn get_token_id(&self, principal_address: &str) -> Option<Token> {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");
        let condition = |event: Registration| -> Option<Token> {
            if event.principal == principal {
                return Some(Token::Uint(event.token_id));
            } else {
                return None;
            }
        };

        let token_id = self.query::<Registration>(condition).await;

        return token_id;
    }

    pub async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<Token> {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");
        let token: U256 = U256::from(token_id);

        let condition = |event: IpfsDeletionRequest| -> Option<Token> {
            if event.principal == principal && event.token_id == token {
                return Some(Token::String(event.ipfs_addr));
            } else {
                return None;
            }
        };

        let ipfs_addr = self.query::<IpfsDeletionRequest>(condition).await;

        return ipfs_addr;
    }

    async fn send(
        &self,
        call: FunctionCall<
            Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
            SignerMiddleware<Provider<Http>, LocalWallet>,
            (),
        >,
    ) -> H256 {
        let tx = call.send().await.unwrap().await.unwrap();
        let tx_hash = tx.unwrap().transaction_hash;

        return tx_hash;
    }

    async fn _call<T>(
        &self,
        func_call: FunctionCall<
            Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
            SignerMiddleware<Provider<Http>, LocalWallet>,
            T,
        >,
    ) -> T
    where
        T: Detokenize,
    {
        let result: T = func_call.call().await.unwrap();
        return result;
    }

    async fn query<T>(&self, condition: impl Fn(T) -> Option<Token>) -> Option<Token>
    where
        T: Detokenize + Event,
    {
        let filter = Filter::new()
            .from_block(0)
            .address(self.contract.address())
            .event(&T::get_signature());
        let http_provider = Provider::<Http>::try_from(&self.api_url).unwrap();
        let logs = http_provider.get_logs(&filter).await.unwrap();

        for log in logs {
            let event = self
                .contract
                .decode_event::<T>(&T::get_name(), log.topics, log.data)
                .unwrap();
            if let Some(thing) = condition(event) {
                return Some(thing);
            }
        }
        return None;
    }
}

#[cfg(test)]
use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;

#[cfg(test)]
#[async_trait]
pub trait Z {
    async fn new(config: &ZksyncConfig) -> MockZksyncClient;
    async fn register_identity(
        &self,
        principal_address: &str,
        ipfs_address: &str,
        data_hash: &str,
    ) -> H256;
    async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256;
    async fn get_token_id(&self, principal_address: &str) -> Option<Token>;
    async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<Token>;
}

#[cfg(test)]
mock! {
    pub ZksyncClient{}
    #[async_trait]
    impl Z for ZksyncClient {
        async fn new(config: &ZksyncConfig) -> MockZksyncClient;
        async fn register_identity(&self, principal_address: &str, ipfs_address: &str, data_hash: &str) -> H256;
        async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256;
        async fn get_token_id(&self, principal_address: &str) -> Option<Token>;
        async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<Token>;
    }
}
