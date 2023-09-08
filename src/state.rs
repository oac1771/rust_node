use serde_json::json;

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

pub async fn create_state(path: &str) {
    let state_json = json!({
        "private_keys": {}
    });

    let state = serde_json::to_string(&state_json).unwrap();
    let parent_directories = path.split("state.json").collect::<Vec<&str>>();

    tokio::fs::create_dir_all(parent_directories[0]).await.unwrap();
    tokio::fs::write(path, state).await.unwrap();
}