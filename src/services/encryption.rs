use openssl::{
    error::ErrorStack,
    rsa::{Padding, Rsa},
};

pub struct EncryptionService {}

impl EncryptionService {
    pub fn new() -> EncryptionService {
        return EncryptionService {};
    }

    pub fn encrypt(&self, content: &str) -> Result<(Vec<u8>, String), ErrorStack> {
        let rsa = Rsa::generate(2048)?;

        let private_key = rsa.private_key_to_pem()?;
        let priv_key_string = String::from_utf8_lossy(&private_key).to_string();

        let mut encrypted_content = vec![0; rsa.size() as usize];
        let encrypted_len =
            rsa.public_encrypt(content.as_bytes(), &mut encrypted_content, Padding::PKCS1)?;
        encrypted_content.truncate(encrypted_len);

        return Ok((encrypted_content, priv_key_string));
    }

    pub fn decrypt(&self, enc_content: Vec<u8>, priv_key: &str) -> Result<Vec<u8>, ErrorStack> {
        let rsa = Rsa::private_key_from_pem(priv_key.as_bytes())?;

        let mut decrypted_content = vec![0; rsa.size() as usize];
        let decryption_len =
            rsa.private_decrypt(&enc_content, &mut decrypted_content, Padding::PKCS1)?;
        decrypted_content.truncate(decryption_len);

        return Ok(decrypted_content);
    }
}
