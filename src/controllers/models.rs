use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub meta_data: String,
    pub data: serde_json::Value 
}