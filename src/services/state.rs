use std::str;

use serde_json::json;

const STATE_PATH: &str = "./var/state.json";

pub struct StateService {}

impl StateService {

    pub async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) {
    
        let mut state = self.read_state().await;
        state["encryption_keys"][principal_address] = encryption_key.into();

        self.write_state(state).await;

    }

    pub async fn create_state(&self) {
        let state_json = json!({
            "encryption_keys": {}
        });
    
        let parent_directories = STATE_PATH.split("state.json").collect::<Vec<&str>>();
        tokio::fs::create_dir_all(parent_directories[0]).await.unwrap();

        self.write_state(state_json).await
    }

    async fn read_state(&self) -> serde_json::Value {
        let state_bytes = tokio::fs::read(STATE_PATH).await.unwrap();
        let state_string = str::from_utf8(&state_bytes).unwrap();
    
        let state = serde_json::from_str::<serde_json::Value>(state_string).unwrap();

        return state
    }

    async fn write_state(&self, state_json: serde_json::Value) {

        let state = serde_json::to_string(&state_json).unwrap();
        tokio::fs::write(STATE_PATH, state).await.unwrap();
    }
}
