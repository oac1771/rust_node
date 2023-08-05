use tempfile::NamedTempFile;
use std::io::Write;

use crate::controllers::models::Data;
use crate::config::Config;

use super::encryption::EncryptionService;
use super::hash::HashService;

pub struct FileService {
    pub encryption_service: EncryptionService,
    pub hash_service: HashService
}

impl FileService {

    pub fn new(config: &Config) -> FileService {

        return FileService {
            encryption_service: EncryptionService::new(config),
            hash_service: HashService::new()
        }
    }

    pub fn create_tempfile(&self, data: Data) -> NamedTempFile {

        let file_content = self.hash_service.hash(data);
        let encrypted_content = self.encryption_service.encrypt(file_content);

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(encrypted_content.as_bytes()).expect("Unable to write to tempfile");

        return temp_file;
    }

}