use std::collections::HashMap;
use std::sync::Mutex;

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