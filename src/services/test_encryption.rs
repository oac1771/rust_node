#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::sync::Mutex;
    use super::super::encryption::EncryptionService;
    use super::super::models::Identity;

    #[test]
    fn test_encrypt_should_return_encrypted_data_and_private_key() {

        let private_key = Mutex::new(HashMap::new());

        let encryption_service = EncryptionService{
            private_keys: &private_key
        };
        
        let file_content = Identity {
            hash: "hash of data".to_string(),
            content: "file content".to_string()
        };

        let (enc_content, _) = encryption_service.encrypt(file_content.to_string());

        assert_ne!(serde_json::to_string(&file_content).unwrap(), enc_content);
        assert_ne!("".to_string(), enc_content);

    }
    
    #[test]
    fn test_should_save_private_key_associated_with_principal_address() {

        let private_keys = Mutex::new(HashMap::new());
        let encryption_service = EncryptionService{
            private_keys: &private_keys
        };
        let principal_address = "principal address";
        let encryption_key = "encryption key";

        encryption_service.save_encryption_key(principal_address, encryption_key);
        let saved_encryption_key = private_keys.lock().unwrap().get(principal_address).unwrap().to_string();

        assert_eq!(saved_encryption_key, encryption_key);

    }

}