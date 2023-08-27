use tempfile::NamedTempFile;
use std::io::Write;

use crate::controllers::models::Data;
use crate::state::State;

use super::encryption::EncryptionService;
use super::hash::HashService;

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

    pub fn generate_identity_file(&self, data: Data) -> (NamedTempFile, String) {

        let identity = self.hash_service.hash(data);
        let (encrypted_content, priv_key) = self.encryption_service.encrypt(&identity);

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(encrypted_content.as_bytes()).expect("Unable to write to tempfile");

        return (temp_file, priv_key);
    }

    pub fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) {
        self.encryption_service.save_encryption_key(principal_address, encryption_key)
    }


}