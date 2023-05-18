use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FileContent {
    meta_data: String,
    data: serde_json::Value 
}