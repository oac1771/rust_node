use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub meta_data: String,
    pub data: serde_json::Value 
}

impl Data {
    pub fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}