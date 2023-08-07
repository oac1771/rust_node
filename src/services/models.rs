use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Identity {
    pub content: String,
    pub hash: String 
}