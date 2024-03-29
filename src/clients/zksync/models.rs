use ethers::{
    types::{U256, Address, H160}, 
    abi::{Detokenize, Token, InvalidOutputType, AbiError},
    providers::ProviderError,
    signers::WalletError,
};
use thiserror::Error;
use rustc_hex::FromHexError;

use super::contracts::ethers_traits::IdentifierError;

pub trait Event {
    fn get_name() -> String;
    fn get_signature() -> String;
}

pub struct Registration {
    pub principal: Address,
    pub token_id: U256
}

impl Registration {

    pub fn new() -> Registration {
        return Registration { principal: H160::zero(), token_id: U256::zero() }
    }

    pub fn set_principal(&mut self, principal: Address) {
        self.principal = principal;
    }

    pub fn set_token_id(&mut self, token_id: U256) {
        self.token_id = token_id;
    }

}

impl Event for Registration {
    fn get_name() -> String {
        return "Registration".to_string()
    }

    fn get_signature() -> String {
        return "Registration(address,uint256)".to_string()
    }
}

impl Detokenize for Registration {

    fn from_tokens(tokens: Vec<Token>) -> Result<Registration, InvalidOutputType> 
    {
        let mut transfer_request = Registration::new();

        for token in tokens {
            match token {
                Token::Address(address) => {
                    let principal = address.clone();
                    transfer_request.set_principal(principal);
                },
                Token::Uint(token_id) => {
                    transfer_request.set_token_id(token_id);
                }
                _ => {return Err(InvalidOutputType("No matching Tokens found".to_string()))}
            }
        }

        return Ok(transfer_request);
    }
}


pub struct IpfsDeletionRequest {
    pub principal: Address,
    pub token_id: U256,
    pub ipfs_addr: String
}

impl IpfsDeletionRequest {

    pub fn new() -> IpfsDeletionRequest {
        return IpfsDeletionRequest { principal: H160::zero(), ipfs_addr: "".to_string(), token_id: U256::zero() }
    }

    pub fn set_principal(&mut self, principal: Address) {
        self.principal = principal;
    }

    pub fn set_ipfs_addr(&mut self, ipfs_addr: String) {
        self.ipfs_addr = ipfs_addr;
    }

    pub fn set_token_id(&mut self, token_id: U256) {
        self.token_id = token_id
    }
}

impl Detokenize for IpfsDeletionRequest {

    fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> 
    {
        let mut ipfs_deletion_request = IpfsDeletionRequest::new();

        for token in tokens {
            match token {
                Token::Address(address) => {
                    let principal = address.clone();
                    ipfs_deletion_request.set_principal(principal);
                },
                Token::String(ipfs_addr) => {
                    ipfs_deletion_request.set_ipfs_addr(ipfs_addr);
                },
                Token::Uint(token_id) => {
                    ipfs_deletion_request.set_token_id(token_id);
                }
                _ => {return Err(InvalidOutputType("No matching Tokens found".to_string()))}
            }
        }

        return Ok(ipfs_deletion_request);
    }
}

impl Event for IpfsDeletionRequest {
    fn get_name() -> String {
        return "IpfsDeletionRequest".to_string()
    }

    fn get_signature() -> String {
        return "IpfsDeletionRequest(address,uint256,string)".to_string()
    }
}

#[derive(Error, Debug)]
pub enum ZksyncClientError {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    WalletError(#[from] WalletError),

    #[error(transparent)]
    FromHexError(#[from] FromHexError),

    #[error(transparent)]
    AbiError(#[from] AbiError),

    #[error(transparent)]
    IdentifierContractError(#[from] IdentifierError),

    #[error("Contract Validation Error: `{0}`")]
    ContractValidationError(String),
}