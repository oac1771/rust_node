use axum::async_trait;
use std::str;
use std::{convert::TryFrom, sync::Arc};

use ethers::{
    abi::{Detokenize, Token},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Filter, H160, U256},
};

use super::contracts::ethers_traits::HttpProvider;
use super::models::ZksyncClientError;
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
    ) -> Result<String, ZksyncClientError>;
    async fn remove_identity(
        &self,
        principal_address: &str,
        token_id: u128,
    ) -> Result<String, ZksyncClientError>;
    async fn check_identity(&self, principal_address: &str) -> Result<bool, ZksyncClientError>;
    async fn get_token_id(&self, principal_address: &str)
        -> Result<Option<u64>, ZksyncClientError>;
    async fn get_ipfs_addr(
        &self,
        principal_address: &str,
        token_id: u128,
    ) -> Result<Option<String>, ZksyncClientError>;
    async fn query<T>(
        &self,
        condition: impl Fn(T) -> Option<Token> + std::marker::Send,
    ) -> Result<Option<Token>, ZksyncClientError>
    where
        T: Detokenize + Event + 'static;
}

pub struct ZksyncClient<I> {
    pub contract: I,
    pub api_url: String,
    pub http_provider: Box<dyn HttpProvider + std::marker::Sync + std::marker::Send>,
}

impl ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    pub async fn new(
        config: &ZksyncConfig,
    ) -> Result<
        ZksyncClient<Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>>,
        ZksyncClientError,
    > {
        if config.contract_address == H160::zero() {
            return Err(ZksyncClientError::ContractValidationError(
                "Contract Address not set".to_string(),
            ));
        }

        let http_provider = Provider::<Http>::try_from(&config.zksync_api_url)?;
        let chain_id = http_provider.get_chainid().await?.as_u64();

        let wallet = config
            .private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id);
        let client = SignerMiddleware::new(http_provider, wallet);

        let contract = Identifier::new(config.contract_address, Arc::new(client));

        return Ok(ZksyncClient {
            contract,
            api_url: config.zksync_api_url.to_string(),
            http_provider: Box::new(Provider::<Http>::try_from(&config.zksync_api_url)?),
        });
    }
}

#[async_trait]
impl<
        I: Iden + std::marker::Sync + std::marker::Send,
    > ZClient for ZksyncClient<I>
{
    async fn register_identity(
        &self,
        principal_address: &str,
        ipfs_address: &str,
        data_hash: &str,
    ) -> Result<String, ZksyncClientError> {
        let principal: Address = principal_address.parse()?;

        let tx_hash = self
            .contract
            .register_identity(principal, ipfs_address.to_string(), data_hash.to_string())
            .await?
            .to_string();

        return Ok(tx_hash);
    }

    async fn remove_identity(
        &self,
        principal_address: &str,
        token_id: u128,
    ) -> Result<String, ZksyncClientError> {
        let principal: Address = principal_address.parse()?;
        let token: U256 = U256::from(token_id);

        let tx_hash = self
            .contract
            .remove_identity(principal, token)
            .await?
            .to_string();

        return Ok(tx_hash);
    }

    async fn check_identity(&self, principal_address: &str) -> Result<bool, ZksyncClientError> {
        let principal: Address = principal_address.parse()?;

        let identity_status = self.contract.check_identity(principal).await?;

        return Ok(identity_status);
    }

    async fn get_token_id(
        &self,
        principal_address: &str,
    ) -> Result<Option<u64>, ZksyncClientError> {
        let principal: Address = principal_address.parse()?;
        let condition = |event: Registration| -> Option<Token> {
            if event.principal == principal {
                return Some(Token::Uint(event.token_id));
            }
            return None;
        };

        let response = if let Some(token) = self.query::<Registration>(condition).await? {
            token.into_uint().map(|x| x.as_u64())
        } else {
            None
        };

        return Ok(response);
    }

    async fn get_ipfs_addr(
        &self,
        principal_address: &str,
        token_id: u128,
    ) -> Result<Option<String>, ZksyncClientError> {
        let principal: Address = principal_address.parse()?;
        let token: U256 = U256::from(token_id);

        let condition = |event: IpfsDeletionRequest| -> Option<Token> {
            if event.principal == principal && event.token_id == token {
                return Some(Token::String(event.ipfs_addr));
            }
            return None;
        };

        let response = if let Some(token) = self.query::<IpfsDeletionRequest>(condition).await? {
            token.into_string()
        } else {
            None
        };

        return Ok(response);
    }

    async fn query<T>(
        &self,
        condition: impl Fn(T) -> Option<Token> + std::marker::Send,
    ) -> Result<Option<Token>, ZksyncClientError>
    where
        T: Detokenize + Event + 'static,
    {
        let filter = Filter::new()
            .from_block(0)
            .address(self.contract.get_address())
            .event(&T::get_signature());
        let logs = self.http_provider.logs(&filter).await?;

        for log in logs {
            let event = self
                .contract
                .decode::<T>(&T::get_name(), log.topics, log.data)?;
            if let Some(token) = condition(event) {
                return Ok(Some(token));
            }
        }
        return Ok(None);
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
pub struct SendExpectation {
    pub func_string: Option<Box<dyn Fn() -> String + std::marker::Sync + std::marker::Send>>,
    pub func_bool: Option<Box<dyn Fn() -> bool + std::marker::Sync + std::marker::Send>>,
}

#[cfg(test)]
pub struct QueryExpectation {
    pub func_string:
        Option<Box<dyn Fn() -> Option<String> + std::marker::Sync + std::marker::Send>>,
    pub func_uint: Option<Box<dyn Fn() -> Option<u64> + std::marker::Sync + std::marker::Send>>,
}

#[cfg(test)]
impl SendExpectation {
    fn new() -> SendExpectation {
        return Self {
            func_string: None,
            func_bool: None,
        };
    }
    pub fn returns_string(
        &mut self,
        func: impl Fn() -> String + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func_string = Some(Box::new(func));
    }

    pub fn _returns_bool(
        &mut self,
        func: impl Fn() -> bool + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func_bool = Some(Box::new(func));
    }
}

#[cfg(test)]
impl QueryExpectation {
    fn new() -> QueryExpectation {
        return Self {
            func_string: None,
            func_uint: None,
        };
    }
    pub fn returns_string(
        &mut self,
        func: impl Fn() -> Option<String> + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func_string = Some(Box::new(func));
    }

    pub fn returns_u64(
        &mut self,
        func: impl Fn() -> Option<u64> + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func_uint = Some(Box::new(func));
    }
}

#[cfg(test)]
impl MockZksyncClient {
    pub fn new() -> Self {
        return Self {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn expect_register_identity(&mut self) -> &mut SendExpectation {
        self.expectations
            .entry("register_identity".to_string())
            .or_insert_with(|| Box::new(SendExpectation::new()))
            .downcast_mut::<SendExpectation>()
            .unwrap()
    }

    pub fn expect_remove_identity(&mut self) -> &mut SendExpectation {
        self.expectations
            .entry("remove_identity".to_string())
            .or_insert_with(|| Box::new(SendExpectation::new()))
            .downcast_mut::<SendExpectation>()
            .unwrap()
    }

    pub fn _expect_check_identity(&mut self) -> &mut SendExpectation {
        self.expectations
            .entry("check_identity".to_string())
            .or_insert_with(|| Box::new(SendExpectation::new()))
            .downcast_mut::<SendExpectation>()
            .unwrap()
    }

    pub fn expect_get_token_id(&mut self) -> &mut QueryExpectation {
        self.expectations
            .entry("get_token_id".to_string())
            .or_insert_with(|| Box::new(QueryExpectation::new()))
            .downcast_mut::<QueryExpectation>()
            .unwrap()
    }

    pub fn expect_get_ipfs_addr(&mut self) -> &mut QueryExpectation {
        self.expectations
            .entry("get_ipfs_addr".to_string())
            .or_insert_with(|| Box::new(QueryExpectation::new()))
            .downcast_mut::<QueryExpectation>()
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
    ) -> Result<String, ZksyncClientError> {
        let expectation = self
            .expectations
            .get("register_identity")
            .unwrap()
            .downcast_ref::<SendExpectation>()
            .unwrap();
        let result = (expectation.func_string.as_ref().unwrap())();

        return Ok(result);
    }

    async fn remove_identity(
        &self,
        _principal_address: &str,
        _token_id: u128,
    ) -> Result<String, ZksyncClientError> {
        let expectation = self
            .expectations
            .get("remove_identity")
            .unwrap()
            .downcast_ref::<SendExpectation>()
            .unwrap();
        let result = (expectation.func_string.as_ref().unwrap())();

        return Ok(result);
    }

    async fn check_identity(&self, _principal_address: &str) -> Result<bool, ZksyncClientError> {
        let expectation = self
            .expectations
            .get("check_identity")
            .unwrap()
            .downcast_ref::<SendExpectation>()
            .unwrap();
        let result = (expectation.func_bool.as_ref().unwrap())();

        return Ok(result);
    }

    async fn get_token_id(
        &self,
        _principal_address: &str,
    ) -> Result<Option<u64>, ZksyncClientError> {
        let expectation = self
            .expectations
            .get("get_token_id")
            .unwrap()
            .downcast_ref::<QueryExpectation>()
            .unwrap();
        let result = (expectation.func_uint.as_ref().unwrap())();

        return Ok(result);
    }

    async fn get_ipfs_addr(
        &self,
        _principal_address: &str,
        _token_id: u128,
    ) -> Result<Option<String>, ZksyncClientError> {
        let expectation = self
            .expectations
            .get("get_ipfs_addr")
            .unwrap()
            .downcast_ref::<QueryExpectation>()
            .unwrap();
        let result = (expectation.func_string.as_ref().unwrap())();

        return Ok(result);
    }
    async fn query<T>(
        &self,
        _condition: impl Fn(T) -> Option<Token> + std::marker::Send,
    ) -> Result<Option<Token>, ZksyncClientError>
    where
        T: Detokenize + Event + 'static,
    {
        return Ok(None);
    }
}
