use ethers::{
    providers::{Provider, Http, Ws, StreamExt, FilterWatcher, Middleware},
    middleware::SignerMiddleware,
    types::{Log, U256, Address},
    contract::{ContractError, EthLogDecode},
    signers::{LocalWallet, Signer},
};

use std::{convert::TryFrom, sync::Arc};

use crate::config::ZksyncConfig;
use crate::identifier::{Identifier, IdentifierEvents};

pub struct ZksyncClient<'a> {
    pub contract: Identifier<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub ws_provider: Provider<Ws>,
    config: &'a ZksyncConfig
}

impl ZksyncClient<'_> {

    pub async fn new<'a>(config: &ZksyncConfig) -> ZksyncClient {

        let wallet = config.private_key.parse::<LocalWallet>().unwrap();

        let ws_provider = Provider::<Ws>::connect(&config.zksync_ws_url).await.unwrap();
        let http_provider = Provider::<Http>::try_from(&config.zksync_api_url).unwrap();

        let client = SignerMiddleware::new(http_provider, wallet);

        let contract = Identifier::new(config.contract_address, Arc::new(client));

        return ZksyncClient{
            contract,
            ws_provider,
            config
        }
    }

    pub async fn get_current_token_id(&self) -> U256 {
        let token_id = self.contract.get_current_token_id().call().await.unwrap();
        return token_id
    }

    pub async fn register_identity(&self, principal: Address, ipfs_address: String, data_hash: String) {

        let call = self.contract.register_identity(principal, ipfs_address, data_hash);
        let receipt = call.send().await;
        println!("{:?}", receipt);

        // match foo {
        //     Err(ContractError::Revert(err)) => {println!("{:?}", err)},
        //     Err(ContractError::DecodingError(err)) => {println!("{:?}", err)},
        //     Err(ContractError::AbiError(err)) => {println!("{:?}", err)},
        //     Err(ContractError::DetokenizationError(err)) => {println!("{:?}", err)},
        //     Err(ContractError::MiddlewareError{e}) => {println!("{:?}", e)},
        //     Err(ContractError::ProviderError{e}) => {println!("{:?}", e)},
        //     Err(ContractError::ContractNotDeployed) => {println!("contract not deployd")},
        //     Err(ContractError::ConstructorError) => {println!("constructor error")},
        //     Ok(U256(val)) => {
        //         println!("No error");
        //         println!("identity registered. Token ID: {:?}", val);
        //     }
        // }
        // self.check_event().await;
    }

    // pub async fn check_event(&self) {
    //     let identifier_ws = Identifier::new(self.config.contract_address, Arc::new(&self.ws_provider));
    //     let events = identifier_ws.events();

    //     let mut http_provider = Provider::<Http>::try_from(self.config.zksync_api_url.to_string()).unwrap();
    //     let block_number = http_provider.get_block_number().await.unwrap();
    //     println!("current block number: {:?}", block_number);

    //     // let mut foo = events.subscribe().await.unwrap();
    //     // let mut bar = events.stream().await.unwrap();

    //     let hi = events.from_block(0).query().await.unwrap();
    //     println!("number of events: {:?}", hi.len());
        
    //     // for event in events.query().await.unwrap() {
    //     //     println!("{:?}", event)
    //     // }
    //     // let mut stream = events.stream().await.unwrap();

    //     // while let Some(Ok(evt)) = foo.next().await {
    //     //     println!("{:?}", evt);
    //     // }
    // }
}