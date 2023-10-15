use std::collections::HashMap;
use serde::{Deserialize, Serialize};

const STATE_PATH: &str = "./var/state.json";

// look into locking file on read and write 
// look into encrypting state.json file somehow
pub struct StateService {}

#[derive(Deserialize, Serialize)]
struct State {
    encryption_keys: HashMap<String, String>
}

impl State {
    pub fn new() -> State {
        return State {
            encryption_keys: HashMap::new()
        }
    }
}

impl StateService {

    pub async fn create_state(&self) {
        let state = State::new();
    
        let parent_directories = STATE_PATH.split("state.json").collect::<Vec<&str>>();
        tokio::fs::create_dir_all(parent_directories[0]).await.unwrap();

        self.write_state(state).await
    }

    async fn read_state(&self) -> State {

        let state_bytes = tokio::fs::read(STATE_PATH).await.unwrap();
        // let state_string = String::from_utf8(state_bytes).unwrap();
    
        let state: State = serde_json::from_slice(&state_bytes).unwrap();

        return state
    }

    async fn write_state(&self, state: impl Serialize) {

        let state_string = serde_json::to_string(&state).unwrap();
        tokio::fs::write(STATE_PATH, state_string).await.unwrap();
    }

    pub async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) {
    
        let mut state = self.read_state().await;
        state.encryption_keys.insert(principal_address.to_lowercase(), encryption_key.to_string());

        self.write_state(state).await;

    }

    // check what happens if principal_address does not exist
    pub async fn get_encryption_key(&self, principal_address: &str) -> Option<String> {
        let state = self.read_state().await;
        let encryption_key = state.encryption_keys.get(principal_address)?;

        return Some(encryption_key.to_string())
    }
}
