use openssl::{
    rsa::{Rsa, Padding}, 
    error::ErrorStack
};

pub struct EncryptionService {}

impl EncryptionService {

    pub fn new() -> EncryptionService {
        return EncryptionService{}
    }

    pub fn encrypt(&self, content: &str) -> (Vec<u8>, String) {

        let rsa = Rsa::generate(2048).unwrap();

        let private_key = rsa.private_key_to_pem().unwrap();
        let priv_key_string = String::from_utf8_lossy(&private_key).to_string();

        let mut encrypted_content = vec![0; rsa.size() as usize];
        let encrypted_len = rsa.public_encrypt(content.as_bytes(), &mut encrypted_content, Padding::PKCS1).unwrap();
        encrypted_content.truncate(encrypted_len);

     
        return (encrypted_content, priv_key_string)
    }

    pub fn decrypt(&self, enc_content: Vec<u8>, priv_key: String) -> Result<Vec<u8>, ErrorStack> {

        let rsa = Rsa::private_key_from_pem(&priv_key.as_bytes().to_vec()).unwrap();

        let mut decrypted_content = vec![0; rsa.size() as usize];
        let decryption = rsa.private_decrypt(&enc_content, &mut decrypted_content, Padding::PKCS1);

        match decryption {
            Ok(decrypted_len) => {
                decrypted_content.truncate(decrypted_len);
                return Ok(decrypted_content)
            },
            Err(err) => {
                return Err(err)
            }
        }

    }

}