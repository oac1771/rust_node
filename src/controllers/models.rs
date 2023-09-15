use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub meta_data: String,
    pub data: serde_json::Value 
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub body: Option<serde_json::Value>,
    pub error: Option<String>
}

impl RegisterResponse {
    pub fn new() -> RegisterResponse {
        return RegisterResponse { body: None, error: None }
    }
    
    pub fn set_error(&mut self, err: String) {
        self.error = Some(err);
    }

    pub fn set_body(&mut self, body: serde_json::Value) {
        self.body = Some(body);
    }
}