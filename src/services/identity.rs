use std::io::Write;
use tempfile::NamedTempFile;

use super::encryption::EncryptionService;
use super::hash::HashService;
use super::models::{Identity, IdentityServiceError};

use crate::utils::bytes_to_string_literal;

pub trait IdService {
    fn create_identity(
        &self,
        data: &str,
    ) -> Result<(NamedTempFile, Identity), IdentityServiceError>;
    fn regenerate_identity(
        &self,
        encryption_key: &str,
        encrypted_data: Vec<u8>,
    ) -> Result<Identity, IdentityServiceError>;
}
pub struct IdentityService {
    pub encryption_service: EncryptionService,
    pub hash_service: HashService,
}

impl IdentityService {
    pub fn new() -> Self {
        return Self {
            encryption_service: EncryptionService::new(),
            hash_service: HashService::new(),
        };
    }
}

impl IdService for IdentityService {
    fn create_identity(
        &self,
        data: &str,
    ) -> Result<(NamedTempFile, Identity), IdentityServiceError> {
        let mut temp_file = NamedTempFile::new()?;

        let hash = self.hash_service.hash(data);

        let (encrypted_content, encryption_key) = self.encryption_service.encrypt(data)?;
        let encrypted_string_literal = bytes_to_string_literal(encrypted_content);

        temp_file.write_all(&encrypted_string_literal.as_bytes())?;

        let identity = Identity {
            hash,
            encryption_key,
            data: encrypted_string_literal,
        };

        return Ok((temp_file, identity));
    }

    fn regenerate_identity(
        &self,
        encryption_key: &str,
        encrypted_data: Vec<u8>,
    ) -> Result<Identity, IdentityServiceError> {
        let decrypted_data = self
            .encryption_service
            .decrypt(encrypted_data, encryption_key)?;

        let data = String::from_utf8(decrypted_data)?;
        let hash = self.hash_service.hash(&data);

        let identity = Identity {
            hash,
            encryption_key: encryption_key.to_string(),
            data,
        };

        return Ok(identity);
    }
}

#[cfg(test)]
pub struct MockIdentityService {
    expectations: std::collections::HashMap<
        String,
        Box<dyn std::any::Any + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
pub struct CreateIdExpectation {
    pub func: Option<
        Box<
            dyn Fn() -> Result<(NamedTempFile, Identity), IdentityServiceError>
                + std::marker::Sync
                + std::marker::Send,
        >,
    >,
}

#[cfg(test)]
pub struct RegenerateIDExpectation {
    pub func: Option<
        Box<
            dyn Fn() -> Result<Identity, IdentityServiceError>
                + std::marker::Sync
                + std::marker::Send,
        >,
    >,
}

#[cfg(test)]
impl CreateIdExpectation {
    pub fn returns(
        &mut self,
        func: impl Fn() -> Result<(NamedTempFile, Identity), IdentityServiceError>
            + 'static
            + std::marker::Sync
            + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }
}

#[cfg(test)]
impl RegenerateIDExpectation {
    pub fn returns(
        &mut self,
        func: impl Fn() -> Result<Identity, IdentityServiceError>
            + 'static
            + std::marker::Sync
            + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }
}

#[cfg(test)]
impl MockIdentityService {
    pub fn new() -> Self {
        return Self {
            expectations: std::collections::HashMap::new(),
        };
    }

    pub fn expect_create_identity(&mut self) -> &mut CreateIdExpectation {
        self.expectations
            .entry("create_id".to_string())
            .or_insert_with(|| Box::new(CreateIdExpectation { func: None }))
            .downcast_mut::<CreateIdExpectation>()
            .unwrap()
    }

    pub fn expect_regenerate_identity(&mut self) -> &mut RegenerateIDExpectation {
        self.expectations
            .entry("regenerate_id".to_string())
            .or_insert_with(|| Box::new(RegenerateIDExpectation { func: None }))
            .downcast_mut::<RegenerateIDExpectation>()
            .unwrap()
    }
}

#[cfg(test)]
impl IdService for MockIdentityService {
    fn create_identity(
        &self,
        _data: &str,
    ) -> Result<(NamedTempFile, Identity), IdentityServiceError> {
        let expectation = self
            .expectations
            .get("create_id")
            .unwrap()
            .downcast_ref::<CreateIdExpectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    fn regenerate_identity(
        &self,
        _encryption_key: &str,
        _encrypted_data: Vec<u8>,
    ) -> Result<Identity, IdentityServiceError> {
        let expectation = self
            .expectations
            .get("regenerate_id")
            .unwrap()
            .downcast_ref::<RegenerateIDExpectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }
}
