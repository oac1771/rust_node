use ethers::{
    abi::{AbiError, Detokenize},
    contract::FunctionCall,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider, ProviderError},
    signers::LocalWallet,
    types::{Address, Bytes, Filter, Log, H256, U256},
};
use std::sync::Arc;

use super::identifier::Identifier;

#[async_trait]
pub trait Iden {
    async fn register_identity(
        &self,
        principal_address: Address,
        ipfs_addaress: String,
        data_hash: String,
    ) -> H256;

    async fn remove_identity(&self, principal_address: Address, token_id: U256) -> H256;

    fn get_address(&self) -> Address;

    async fn check_identity(&self, principal_address: Address) -> bool;

    fn decode<D>(&self, name: &str, topics: Vec<H256>, data: Bytes) -> Result<D, AbiError>
    where
        D: Detokenize + 'static;
}

#[async_trait]
impl Iden for Identifier<SignerMiddleware<Provider<Http>, LocalWallet>> {
    async fn register_identity(
        &self,
        principal_address: Address,
        ipfs_addaress: String,
        data_hash: String,
    ) -> H256 {
        let call = self.register_identity(principal_address, ipfs_addaress, data_hash);
        let tx_hash = self.send(call).await;
        return tx_hash;
    }

    async fn remove_identity(&self, principal_address: Address, token_id: U256) -> H256 {
        let call = self.remove_identity(token_id, principal_address);
        let tx_hash = self.send(call).await;
        return tx_hash;
    }

    fn get_address(&self) -> Address {
        return self.address();
    }

    async fn check_identity(&self, principal_address: Address) -> bool {
        let call = self.check_identity(principal_address);
        return self.call::<bool>(call).await;
    }

    fn decode<D>(&self, name: &str, topics: Vec<H256>, data: Bytes) -> Result<D, AbiError>
    where
        D: Detokenize,
    {
        return self.decode_event::<D>(name, topics, data);
    }
}

impl Identifier<SignerMiddleware<Provider<Http>, LocalWallet>> {
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
    async fn call<T>(
        &self,
        call: FunctionCall<
            Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
            SignerMiddleware<Provider<Http>, LocalWallet>,
            T,
        >,
    ) -> T
    where
        T: Detokenize,
    {
        let result: T = call.call().await.unwrap();
        return result;
    }
}

#[async_trait]
pub trait HttpProvider {
    async fn logs(&self, filter: &Filter) -> Result<Vec<Log>, ProviderError>;
}

#[async_trait]
impl HttpProvider for Provider<Http> {
    async fn logs(&self, filter: &Filter) -> Result<Vec<Log>, ProviderError> {
        return self.get_logs(filter).await;
    }
}

#[cfg(test)]
pub struct MockIdentifier {
    expectations: std::collections::HashMap<
        String,
        Box<dyn std::any::Any + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
pub struct Expectation {
    pub func: Option<Box<dyn Fn() -> H256 + std::marker::Sync + std::marker::Send>>,
    pub address: Option<Address>,
    pub val: bool,
}

#[cfg(test)]
impl Expectation {
    pub fn new() -> Expectation {
        return Expectation {
            func: None,
            address: None,
            val: true,
        };
    }
    pub fn returns(
        &mut self,
        func: impl Fn() -> H256 + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }

    pub fn returns_const(&mut self, address: Address) {
        self.address = Some(address)
    }

    pub fn returns_bool(&mut self, val: bool) {
        self.val = val
    }
}

#[cfg(test)]
pub struct DecodeExpectation<D> {
    pub func: Option<Box<dyn Fn() -> Result<D, AbiError> + std::marker::Sync + std::marker::Send>>,
}

#[cfg(test)]
impl<D: Detokenize> DecodeExpectation<D> {
    pub fn new() -> DecodeExpectation<D> {
        return Self { func: None };
    }

    pub fn returns(
        &mut self,
        func: impl Fn() -> Result<D, AbiError> + std::marker::Sync + std::marker::Send + 'static,
    ) {
        self.func = Some(Box::new(func))
    }
}

#[cfg(test)]
impl MockIdentifier {
    pub fn new() -> Self {
        return Self {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn expect_decode<D>(&mut self) -> &mut DecodeExpectation<D>
    where
        D: Detokenize + std::marker::Sync + std::marker::Send + 'static,
    {
        self.expectations
            .entry("decode".to_string())
            .or_insert_with(|| Box::new(DecodeExpectation::<D>::new()))
            .downcast_mut::<DecodeExpectation<D>>()
            .unwrap()
    }

    pub fn expect_remove_identity(&mut self) -> &mut Expectation {
        self.expectations
            .entry("remove_identity".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }

    pub fn expect_register_identity(&mut self) -> &mut Expectation {
        self.expectations
            .entry("register_identity".to_string())
            .or_insert_with(|| Box::new(Expectation::new()))
            .downcast_mut::<Expectation>()
            .unwrap()
    }

    pub fn expect_get_address(&mut self) -> &mut Expectation {
        self.expectations
            .entry("get_address".to_string())
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
}

#[cfg(test)]
#[async_trait]
impl Iden for MockIdentifier {
    async fn register_identity(
        &self,
        _principal_address: Address,
        _ipfs_addaress: String,
        _data_hash: String,
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

    async fn remove_identity(&self, _principal_address: Address, _token_id: U256) -> H256 {
        let expectation = self
            .expectations
            .get("remove_identity")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    fn get_address(&self) -> Address {
        let expectation = self
            .expectations
            .get("get_address")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = expectation.address.unwrap();

        return result;
    }

    async fn check_identity(&self, _principal_address: Address) -> bool {
        let expectation = self
            .expectations
            .get("check_identity")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = expectation.val;
        return result
    }

    fn decode<D>(&self, _name: &str, _topics: Vec<H256>, _data: Bytes) -> Result<D, AbiError>
    where
        D: Detokenize + 'static,
    {
        let expectation = self
            .expectations
            .get("decode")
            .unwrap()
            .downcast_ref::<DecodeExpectation<D>>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }
}

#[cfg(test)]
pub struct MockHttpProvider {
    expectations: std::collections::HashMap<
        String,
        Box<dyn std::any::Any + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
pub struct HttpProviderExpectation {
    pub func: Option<
        Box<dyn Fn() -> Result<Vec<Log>, ProviderError> + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
impl HttpProviderExpectation {
    pub fn new() -> Self {
        return Self { func: None };
    }

    pub fn returns(
        &mut self,
        func: impl Fn() -> Result<Vec<Log>, ProviderError>
            + 'static
            + std::marker::Sync
            + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }
}

#[cfg(test)]
impl MockHttpProvider {
    pub fn new() -> Self {
        return Self {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn expect_logs(&mut self) -> &mut HttpProviderExpectation {
        self.expectations
            .entry("logs".to_string())
            .or_insert_with(|| Box::new(HttpProviderExpectation::new()))
            .downcast_mut::<HttpProviderExpectation>()
            .unwrap()
    }
}

#[cfg(test)]
#[async_trait]
impl HttpProvider for MockHttpProvider {
    async fn logs(&self, _filter: &Filter) -> Result<Vec<Log>, ProviderError> {
        let expectation = self
            .expectations
            .get("logs")
            .unwrap()
            .downcast_ref::<HttpProviderExpectation>()
            .unwrap();

        let result = (expectation.func.as_ref().unwrap())();
        return result;
    }
}
