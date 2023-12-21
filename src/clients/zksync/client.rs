use std::str;
use std::{convert::TryFrom, sync::Arc};

use ethers::{
    abi::{Detokenize, Token},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Filter, H256, U256},
};

use super::contracts::ethers_traits::HttpProvider;
use super::{
    contracts::{ethers_traits::Iden, identifier::Identifier},
    models::{Event, IpfsDeletionRequest, Registration},
};

use crate::services::config::ZksyncConfig;

#[async_trait]
pub trait ZClient {
    async fn register_identity(
        &self,
        principal_address: &str,
        ipfs_address: &str,
        data_hash: &str,
    ) -> H256;
    async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256;
    async fn check_identity(&self, principal_address: &str) -> bool;
    async fn get_token_id(&self, principal_address: &str) -> Option<Token>;
    async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<Token>;
    async fn query<T>(
        &self,
        condition: impl Fn(T) -> Option<Token> + std::marker::Send,
    ) -> Option<Token>
    where
        T: Detokenize + Event + 'static;
}

pub struct ZksyncClient<I, H> {
    pub contract: I,
    pub api_url: String,
    pub http_provider: H,
}

impl ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>, Provider<Http>> {
    pub async fn new(
        config: &ZksyncConfig,
    ) -> ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>, Provider<Http>>
    {
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
            http_provider: Provider::<Http>::try_from(&config.zksync_api_url).unwrap(),
        };
    }
}

#[async_trait]
impl<
        I: Iden + std::marker::Sync + std::marker::Send,
        H: HttpProvider + std::marker::Sync + std::marker::Send,
    > ZClient for ZksyncClient<I, H>
{
    async fn register_identity(
        &self,
        principal_address: &str,
        ipfs_address: &str,
        data_hash: &str,
    ) -> H256 {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");

        let tx_hash = self
            .contract
            .register_identity(principal, ipfs_address.to_string(), data_hash.to_string())
            .await;

        return tx_hash;
    }

    async fn remove_identity(&self, principal_address: &str, token_id: u128) -> H256 {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");
        let token: U256 = U256::from(token_id);

        let tx_hash = self.contract.remove_identity(principal, token).await;

        return tx_hash;
    }

    async fn check_identity(&self, principal_address: &str) -> bool {
        let principal: Address = principal_address
            .parse()
            .expect("Invalid principal address");

        let identity_status = self.contract.check_identity(principal).await;

        return identity_status;
    }

    async fn get_token_id(&self, principal_address: &str) -> Option<Token> {
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

    async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<Token> {
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

    async fn query<T>(
        &self,
        condition: impl Fn(T) -> Option<Token> + std::marker::Send,
    ) -> Option<Token>
    where
        T: Detokenize + Event + 'static,
    {
        let filter = Filter::new()
            .from_block(0)
            .address(self.contract.get_address())
            .event(&T::get_signature());
        let logs = self.http_provider.logs(&filter).await.unwrap();

        for log in logs {
            let event = self
                .contract
                .decode::<T>(&T::get_name(), log.topics, log.data)
                .unwrap();
            if let Some(token) = condition(event) {
                return Some(token);
            }
        }
        return None;
    }
}

#[cfg(test)]
pub struct MockZksyncClient {
    expectations: std::collections::HashMap<
        String,
        Box<dyn std::any::Any + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
pub struct Expectation {
    pub func: Option<Box<dyn Fn() -> H256 + std::marker::Sync + std::marker::Send>>,
    pub token: Option<Box<dyn Fn() -> Option<Token> + std::marker::Sync + std::marker::Send>>,
    pub val: bool,
}

#[cfg(test)]
impl Expectation {
    fn new() -> Expectation {
        return Self {
            func: None,
            val: false,
            token: None,
        };
    }
    pub fn returns(
        &mut self,
        func: impl Fn() -> H256 + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }

    pub fn returns_token(
        &mut self,
        func: impl Fn() -> Option<Token> + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.token = Some(Box::new(func));
    }
}

#[cfg(test)]
impl MockZksyncClient {
    pub fn new() -> Self {
        return Self {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn expect_register_identity(&mut self) -> &mut Expectation {
        self.expectations
            .entry("register_identity".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }

    pub fn expect_remove_identity(&mut self) -> &mut Expectation {
        self.expectations
            .entry("remove_identity".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }

    pub fn expect_check_identity(&mut self) -> &mut Expectation {
        self.expectations
            .entry("check_identity".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }

    pub fn expect_get_token_id(&mut self) -> &mut Expectation {
        self.expectations
            .entry("get_token_id".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }

    pub fn expect_get_ipfs_addr(&mut self) -> &mut Expectation {
        self.expectations
            .entry("get_ipfs_addr".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }
}

#[cfg(test)]
#[async_trait]
impl ZClient for MockZksyncClient {
    async fn register_identity(
        &self,
        _principal_address: &str,
        _ipfs_address: &str,
        _data_hash: &str,
    ) -> H256 {
        let expectation = self
            .expectations
            .get("register_identity")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    async fn remove_identity(&self, _principal_address: &str, _token_id: u128) -> H256 {
        let expectation = self
            .expectations
            .get("remove_identity")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    async fn check_identity(&self, _principal_address: &str) -> bool {
        let expectation = self
            .expectations
            .get("check_identity")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = expectation.val;

        return result;
    }

    async fn get_token_id(&self, _principal_address: &str) -> Option<Token> {
        let expectation = self
            .expectations
            .get("get_token_id")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.token.as_ref().unwrap())();

        return result;
    }

    async fn get_ipfs_addr(&self, _principal_address: &str, _token_id: u128) -> Option<Token> {
        let expectation = self
            .expectations
            .get("get_ipfs_addr")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.token.as_ref().unwrap())();

        return result;
    }
    async fn query<T>(
        &self,
        _condition: impl Fn(T) -> Option<Token> + std::marker::Send,
    ) -> Option<Token>
    where
        T: Detokenize + Event + 'static,
    {
        return None;
    }
}
