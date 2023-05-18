use crate::clients::ipfs::client::IpfsClient;
use crate::services::file::FileService;

use super::models::FileContent;

pub struct AddController {
    pub ipfs_client: IpfsClient,
    pub file_service: FileService
}

impl AddController {

    pub fn new() -> AddController {
        let ipfs_client: IpfsClient = IpfsClient::new();
        let add_controller = AddController {
            ipfs_client,
            file_service: FileService{}
        };
        return add_controller
    }

    pub async fn add(&self, file_contents: FileContent) -> String {
        let content = serde_json::to_string(&file_contents).unwrap();
        let temp_file = self.file_service.create_tempfile(content);
        let response = self.ipfs_client.add_file(&temp_file.path().to_str().unwrap()).await;

        return response
    }
}