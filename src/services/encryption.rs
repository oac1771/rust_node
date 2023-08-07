use std::collections::HashMap;
use std::sync::Mutex;
use openssl::rsa::{Rsa, Padding};

use crate::config::EncryptionConfig;

use super::models::Identity;

pub struct EncryptionService<'a> {
    pub private_keys: &'a Mutex<HashMap<String, String>>
}

impl<'a> EncryptionService<'a> {

    pub fn new(config: &EncryptionConfig ) -> EncryptionService {
        return EncryptionService{
            private_keys: &config.private_keys
        }
    }

    pub fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) -> Option<bool>{

        if self.private_keys.lock().unwrap().contains_key(&principal_address.to_string()) != true {
            self.private_keys.lock().unwrap().insert(principal_address.to_string(), encryption_key.to_string());
            return Some(true)
        } else {
            return None
        }
        
    }

    pub fn encrypt(&self, identity: &Identity) -> (String, String) {

        let content = serde_json::to_string(&identity).unwrap();
        let rsa = Rsa::generate(2048).unwrap();

        let private_key = rsa.private_key_to_pem().unwrap();
        let priv_key_string = String::from_utf8_lossy(&private_key).to_string();

        let mut encrypted_content = vec![0; rsa.size() as usize];
        let encrypted_len = rsa.public_encrypt(content.as_bytes(), &mut encrypted_content, Padding::PKCS1).unwrap();
        encrypted_content.truncate(encrypted_len);

        let enc_content_string = String::from_utf8_lossy(&encrypted_content).to_string();
     
        return (enc_content_string, priv_key_string)
    }

}