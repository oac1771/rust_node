use serde::Serialize;


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