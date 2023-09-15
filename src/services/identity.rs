use tempfile::NamedTempFile;
use std::io::Write;
use serde::Serialize;

use super::encryption::EncryptionService;
use super::hash::HashService;
use super::models::Identity;

pub struct IdentityService {
    pub encryption_service: EncryptionService,
    pub hash_service: HashService,
}

// generate_identity_file should take in data, encrypt it and return (NamedTempFile, Identity)
    // instead of calling these two functions
    // Identity struct should hold hash, priv_key, content
    // to_string() method should use json!() to write content and hash to encryption service

impl IdentityService {

    pub fn new() -> IdentityService {

        return IdentityService {
            encryption_service: EncryptionService::new(),
            hash_service: HashService::new(),
        }
    }
 
    pub fn encrypt_file_contents(&self, data: impl Serialize, temp_file: &mut NamedTempFile) -> (String, String) {

        let (content, hash) = self.hash_service.hash(data);

        let identity = Identity::new(content, hash);

        let (encrypted_content, priv_key) = self.encryption_service.encrypt(identity.to_string());

        temp_file.write_all(&encrypted_content).expect("Unable to write to tempfile");

        return (identity.hash.to_string(), priv_key);
    }

    pub fn generate_identity_file(&self) -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        return temp_file
    }

}