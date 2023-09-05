use std::collections::HashMap;
use std::sync::Mutex;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub status: String
}

pub struct State {
    pub encryption_state: EncryptionState,
}

pub struct EncryptionState {
    pub private_keys: Mutex<HashMap<String, String>>
}

pub fn set_state() -> State {
    let encryption_state = EncryptionState {
        private_keys: Mutex::new(HashMap::new())
    };


    return State {
        encryption_state,
    }
}