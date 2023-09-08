#[cfg(test)]
mod tests {

    use super::super::encryption::EncryptionService;
    use super::super::models::Identity;

    #[test]
    fn test_encrypt_should_return_encrypted_data_and_private_key() {

        let encryption_service = EncryptionService{};
        
        let file_content = Identity {
            hash: "hash of data".to_string(),
            content: "file content".to_string()
        };

        let (enc_content, _) = encryption_service.encrypt(file_content.to_string());

        assert_ne!(serde_json::to_string(&file_content).unwrap(), enc_content);
        assert_ne!("".to_string(), enc_content);

    }

}