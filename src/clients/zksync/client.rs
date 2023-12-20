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
    async fn get_token_id(&self, principal_address: &str) -> Option<Token>;
    async fn get_ipfs_addr(&self, principal_address: &str, token_id: u128) -> Option<Token>;
    async fn check_identity(&self, principal_address: &str) -> bool;
    async fn query<T>(&self, condition: impl Fn(T) -> Option<Token> + std::marker::Send) -> Option<Token>
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

    async fn query<T>(&self, condition: impl Fn(T) -> Option<Token> + std::marker::Send) -> Option<Token>
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
