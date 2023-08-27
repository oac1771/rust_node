use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::config::Config;
use crate::state::State;
use crate::services::identity::IdentityService;

use super::models::Data;

pub struct RegisterController<'a> {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub identity_service: IdentityService<'a>
}

impl<'a> RegisterController<'a> {

    pub async fn new(config: &'a Config, state: &'a State) -> RegisterController<'a> {
        
        let ipfs_client = IpfsClient::new(&config.ipfs_config);
        let identity_service = IdentityService::new(&state);
        let zksync_client = ZksyncClient::new(&config.zksync_config).await;

        let register_controller = RegisterController {
            ipfs_client,
            zksync_client,
            identity_service
        };
        return register_controller
    }

    // call contract check_identity to see if exists before doing all of this
    pub async fn register(&self, data: Data, principal_address: &str) -> String {

        let (identity_file, encryption_key) = self.identity_service.generate_identity_file(data);
        let identity_file_path = identity_file.path().to_str().unwrap().to_string();

        let response = self.ipfs_client.add_file(&identity_file_path).await;

        self.identity_service.save_encryption_key(principal_address, &encryption_key);

        return response

    }
}