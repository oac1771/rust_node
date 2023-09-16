use crypto::{sha2::Sha256, digest::Digest};

pub struct HashService{}

impl HashService {

    pub fn new() -> HashService {
        return HashService {}
    }

    pub fn hash(&self, data: &str) -> String {

        let mut hasher = Sha256::new();
        hasher.input_str(data);

        return hasher.result_str()

    }

}