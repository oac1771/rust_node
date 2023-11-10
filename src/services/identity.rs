use std::io::Write;
use tempfile::NamedTempFile;

use super::encryption::EncryptionService;
use super::hash::HashService;
use super::models::{Identity, IdentityServiceError};

use crate::utils::{bytes_to_string_literal, string_literal_to_bytes};

pub struct IdentityService {
    pub encryption_service: EncryptionService,
    pub hash_service: HashService,
}

impl IdentityService {
    pub fn new() -> IdentityService {
        return IdentityService {
            encryption_service: EncryptionService::new(),
            hash_service: HashService::new(),
        };
    }

    pub fn create_identity(
        &self,
        data: &str,
    ) -> Result<(NamedTempFile, Identity), IdentityServiceError> {
        let mut temp_file = NamedTempFile::new()?;

        let hash = self.hash_service.hash(data);

        let (encrypted_content, encryption_key) = self.encryption_service.encrypt(data)?;
        let encrypted_string_literal = bytes_to_string_literal(encrypted_content);
        println!("key: {:?}", encryption_key);

        temp_file.write_all(&encrypted_string_literal.as_bytes())?;

        let identity = Identity {
            hash,
            encryption_key,
            data: encrypted_string_literal
        };

        return Ok((temp_file, identity));
    }

    pub fn regenerate_identity(
        &self,
        encryption_key: &str,
        encrypted_data: &str,
    ) -> Result<Identity, IdentityServiceError> {

        let encrypted_bytes = string_literal_to_bytes(&encrypted_data);

        if let None = encrypted_bytes {
            return Err(IdentityServiceError{err: "Unable to transform string literal to bytes".to_string()})
        }

        let decrypted_data = self
            .encryption_service
            .decrypt(encrypted_bytes.unwrap(), encryption_key)?;

        let data = String::from_utf8(decrypted_data)?;
        let hash = self.hash_service.hash(&data);

        let identity = Identity {
            hash,
            encryption_key: encryption_key.to_string(),
            data
        };

        return Ok(identity);
    }
}
