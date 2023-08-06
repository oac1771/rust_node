use crate::clients::ipfs::client::IpfsClient;
use crate::clients::zksync::client::ZksyncClient;
use crate::config::Config;
use crate::services::file::FileService;

use super::models::Data;

pub struct RegisterController<'a> {
    pub ipfs_client: IpfsClient,
    pub zksync_client: ZksyncClient,
    pub file_service: FileService<'a>
}

impl<'a> RegisterController<'a> {

    pub fn new(config: &Config) -> RegisterController {
        
        let ipfs_client: IpfsClient = IpfsClient::new(&config.ipfs_config);
        let file_service: FileService = FileService::new(config);

        let register_controller = RegisterController {
            ipfs_client,
            zksync_client: ZksyncClient{},
            file_service
        };
        return register_controller
    }

    pub async fn register(&self, data: Data, principal_address: &str) -> String {

        let temp_file = self.file_service.create_tempfile(data, principal_address);
        let file_name = temp_file.path().to_str().unwrap().to_string();

        let response = self.ipfs_client.add_file(&file_name).await;

        return response
    }
}