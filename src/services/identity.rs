use tempfile::NamedTempFile;
use std::io::Write;

use super::encryption::EncryptionService;
use super::hash::HashService;
use super::models::Identity;

use crate::utils::bytes_to_string_literal;

pub struct IdentityService {
    pub encryption_service: EncryptionService,
    pub hash_service: HashService,
}

impl IdentityService {

    pub fn new() -> IdentityService {

        return IdentityService {
            encryption_service: EncryptionService::new(),
            hash_service: HashService::new(),
        }
    }

    pub fn generate_identity(&self, data: &str) -> (NamedTempFile, Identity) {

        let mut temp_file = NamedTempFile::new().unwrap();

        let hash = self.hash_service.hash(data);

        let (encrypted_content, encryption_key) = self.encryption_service.encrypt(data);
        let encrypted_string_literal = bytes_to_string_literal(encrypted_content);
        temp_file.write_all(&encrypted_string_literal.as_bytes()).expect("Unable to write to tempfile");

        let identity = Identity{hash, encryption_key};

        return (temp_file, identity)
    }

}