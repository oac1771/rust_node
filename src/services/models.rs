use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FileContent {
    pub content: String,
    pub hash: String 
}