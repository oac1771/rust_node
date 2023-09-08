use openssl::rsa::{Rsa, Padding};

pub struct EncryptionService {}

impl EncryptionService {

    pub fn new() -> EncryptionService {
        return EncryptionService{}
    }

    pub fn encrypt(&self, content: String) -> (String, String) {

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