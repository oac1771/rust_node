use serde::Serialize;
use serde_json::Error;

use ethers::{
    contract::ContractError,
    providers::{Provider, ProviderError, Ws},
};

use rustc_hex::FromHexError;
use tokio::task::JoinError;

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
        token_id: Option<u64>,
        ipfs_address: String,
        encryption_key: String,
    ) -> Result<Self, RegisterError> {
        if let Some(token) = token_id {
            return Ok(Self {
                tx_hash: tx_hash,
                token_id: token,
                ipfs_address,
                encryption_key,
            });
        } else {
            return Err(RegisterError {
                err: "Unable to read TokenID".to_string(),
            });
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

#[derive(Serialize, Debug)]
pub struct RegisterError {
    pub err: String,
}

impl From<ZksyncClientError> for RegisterError {
    fn from(error: ZksyncClientError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<StateServiceError> for RegisterError {
    fn from(error: StateServiceError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<IdentityServiceError> for RegisterError {
    fn from(error: IdentityServiceError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<IpfsClientError> for RegisterError {
    fn from(error: IpfsClientError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<Error> for RegisterError {
    fn from(error: Error) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl std::fmt::Display for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

#[derive(Serialize, Debug)]
pub struct AuthenticationError {
    pub err: String,
}

impl From<ProviderError> for AuthenticationError {
    fn from(error: ProviderError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<FromHexError> for AuthenticationError {
    fn from(error: FromHexError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<ContractError<Provider<Ws>>> for AuthenticationError {
    fn from(error: ContractError<Provider<Ws>>) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<JoinError> for AuthenticationError {
    fn from(error: JoinError) -> Self {
        Self {
            err: error.to_string(),
        }
    }
}

impl From<IdentityServiceError> for AuthenticationError {
    fn from(err: IdentityServiceError) -> AuthenticationError {
        return Self {
            err: err.to_string(),
        };
    }
}

impl From<StateServiceError> for AuthenticationError {
    fn from(err: StateServiceError) -> AuthenticationError {
        return Self {
            err: err.to_string(),
        };
    }
}

impl From<IpfsClientError> for AuthenticationError {
    fn from(err: IpfsClientError) -> AuthenticationError {
        return Self {
            err: err.to_string(),
        };
    }
}
