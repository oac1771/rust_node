#[cfg(test)]
mod tests {

    use super::super::encryption::EncryptionService;
    use super::super::models::Data;
    use serde_json::json;

    #[test]
    fn encrypt_should_return_encrypted_data_and_private_key() {

        let encryption_service = EncryptionService{};
        
        let content = Data {
            meta_data: "hash of data".to_string(),
            data: json!({"foo": "bar"})
        };

        let (enc_content, _) = encryption_service.encrypt(&content.to_string());

        assert_ne!(serde_json::to_string(&content).unwrap().as_bytes(), enc_content);
        assert_ne!("".to_string().as_bytes(), enc_content);

    }

    #[test]
    fn decrypt_should_return_original_data() {
        let encryption_service = EncryptionService{};
        
        let content = Data {
            meta_data: "hash of data".to_string(),
            data: json!({"foo": "bar"})
        };

        let (enc_content, priv_key) = encryption_service.encrypt(&content.to_string());
        let decrypted_content = encryption_service.decrypt(enc_content, priv_key);

        assert_eq!(content.to_string().as_bytes(), decrypted_content.unwrap());
    }

}