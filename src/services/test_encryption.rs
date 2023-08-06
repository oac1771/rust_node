#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::sync::Mutex;
    use super::super::encryption::EncryptionService;
    use super::super::models::FileContent;

    #[test]
    fn test_encrypt_should_return_encrypted_data_and_private_key() {

        let private_key = Mutex::new(HashMap::new());

        let encryption_service = EncryptionService{
            private_keys: &private_key
        };
        
        let file_content = FileContent {
            hash: "hash of data".to_string(),
            content: "file content".to_string()
        };
        let principal_address = "principal address";

        let (enc_content, _) = encryption_service.encrypt(&file_content, principal_address);

        assert_ne!(serde_json::to_string(&file_content).unwrap(), enc_content);

    }
    
}