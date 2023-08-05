use crate::config::Config;

use super::models::FileContent;

pub struct EncryptionService {}

impl EncryptionService {

    pub fn new(config: &Config) -> EncryptionService {
        return EncryptionService{}
    }

    pub fn encrypt(&self, file_content: FileContent) -> String {
        return "contnts".to_string();
    }

}