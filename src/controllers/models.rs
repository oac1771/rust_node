use serde::Serialize;

use ethers::{
    contract::ContractError,
    providers::{Provider, ProviderError, Ws},
};

use crate::services::models::IdentityServiceError;
use crate::{
    clients::{ipfs::models::IpfsClientError, zksync::models::ZksyncClientError},
    services::models::StateServiceError,
};

#[derive(Serialize)]
pub struct Health {
    pub status: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub tx_hash: String,
    pub token_id: u64,
    pub ipfs_address: String,
    pub encryption_key: String,
}

#[derive(Serialize)]
pub struct RemoveResponse {
    pub tx_hash: String,
    pub removed_pins: Vec<String>,
}

impl RegisterResponse {
    pub fn new(
        tx_hash: String,
        token_id: u64,
        ipfs_address: String,
        encryption_key: String,
    ) -> Self {
        Self {
            tx_hash: tx_hash,
            token_id: token_id,
            ipfs_address,
            encryption_key,
        }
    }
}

impl RemoveResponse {
    pub fn new(tx_hash: String, removed_pins: Vec<String>) -> Self {
        return Self {
            tx_hash,
            removed_pins,
        };
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RegisterError {
    #[error(transparent)]
    ZksyncClientError(#[from] ZksyncClientError),

    #[error(transparent)]
    StateServiceError(#[from] StateServiceError),

    #[error(transparent)]
    IdentityServiceError(#[from] IdentityServiceError),

    #[error(transparent)]
    IpfsClientError(#[from] IpfsClientError),

    #[error(transparent)]
    SerdeSerializeError(#[from] serde_json::Error),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),

    #[error("`{0}`")]
    OtherError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error(transparent)]
    StateServiceError(#[from] StateServiceError),

    #[error(transparent)]
    IdentityServiceError(#[from] IdentityServiceError),

    #[error(transparent)]
    IpfsClientError(#[from] IpfsClientError),

    #[error(transparent)]
    SerdeSerializeError(#[from] serde_json::Error),

    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    IdentityContractError(#[from] ContractError<Provider<Ws>>),

    #[error("`{0}`")]
    OtherError(String),
}
