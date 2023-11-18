use async_trait::async_trait;
use ethers::{
    contract::FunctionCall,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    types::{Address, U256},
    signers::LocalWallet
};
use std::sync::Arc;

use super::identifier::Identifier;

#[async_trait]
pub trait Iden<T> {
    fn register_identity(
        &self,
        principal_address: Address,
        ipfs_addaress: String,
        data_hash: String,
    ) -> FunctionCall<Arc<T>, T, ()>;

    fn remove_identity(&self, principal_address: Address, token_id: U256) -> FunctionCall<Arc<T>, T, ()>;
}

#[async_trait]
impl Iden<SignerMiddleware<Provider<Http>, LocalWallet>> for Identifier<SignerMiddleware<Provider<Http>, LocalWallet>> {
    fn register_identity(
        &self,
        principal_address: Address,
        ipfs_addaress: String,
        data_hash: String,
    ) -> FunctionCall<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, SignerMiddleware<Provider<Http>, LocalWallet>, ()> {
        return self.register_identity(principal_address, ipfs_addaress, data_hash);
    }

    fn remove_identity(
        &self,
        principal_address: Address,
        token_id: U256
    ) -> FunctionCall<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, SignerMiddleware<Provider<Http>, LocalWallet>, ()> {
        return self.remove_identity(token_id, principal_address);
    }
}