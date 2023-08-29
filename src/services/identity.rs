use tempfile::NamedTempFile;
use std::io::Write;

use crate::controllers::models::Data;
use crate::state::State;

use super::encryption::EncryptionService;
use super::hash::HashService;
use super::models::Identity;

pub struct IdentityService<'a> {
    pub encryption_service: EncryptionService<'a>,
    pub hash_service: HashService
}

impl<'a> IdentityService<'a> {

    pub fn new(state: &State) -> IdentityService {

        return IdentityService {
            encryption_service: EncryptionService::new(&state.encryption_state),
            hash_service: HashService::new()
        }
    }
 
    pub fn encrypt_file_contents(&self, data: Data, temp_file: &mut NamedTempFile) -> (String, String) {

        let (content, hash) = self.hash_service.hash(data);
        let identity = Identity::new(content, hash);

        let (encrypted_content, priv_key) = self.encryption_service.encrypt(identity.to_string());

        temp_file.write_all(encrypted_content.as_bytes()).expect("Unable to write to tempfile");

        return (identity.hash.to_string(), priv_key);
    }

    pub fn generate_identity_file(&self) -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        return temp_file
    }

    pub fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) {
        self.encryption_service.save_encryption_key(principal_address, encryption_key)
    }


}