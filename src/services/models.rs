use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Identity {
    pub content: String,
    pub hash: String 
}

impl Identity {
    pub fn new(content: String, hash: String) -> Identity {
        return Identity {content, hash}
    }

    pub fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}