use crypto::{sha2::Sha256, digest::Digest};
use crate::controllers::models::Data;

pub struct HashService{}

impl HashService {

    pub fn new() -> HashService {
        return HashService {}
    }

    pub fn hash(&self, data: Data) -> (String, String) {
        let content = serde_json::to_string(&data).unwrap();

        let mut hasher = Sha256::new();
        hasher.input_str(&content);

        return (content, hasher.result_str())

    }

}