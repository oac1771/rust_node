use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STATE_PATH: &str = "./var/state.json";

pub struct StateService {}

#[derive(Deserialize, Serialize)]
pub struct State {
    encryption_keys: HashMap<String, String>,
}

impl State {
    pub fn new() -> State {
        return State {
            encryption_keys: HashMap::new(),
        };
    }
}

impl StateService {

    pub fn new() -> Self {
        return Self{}
    }

    pub async fn create_state(&self) {
        let state = State::new();

        let parent_directories = STATE_PATH.split("state.json").collect::<Vec<&str>>();
        tokio::fs::create_dir_all(parent_directories[0])
            .await
            .unwrap();

        self.write_state(state).await
    }

    async fn read_state(&self) -> State {
        let state_bytes = tokio::fs::read(STATE_PATH).await.unwrap();

        let state: State = serde_json::from_slice(&state_bytes).unwrap();

        return state;
    }

    async fn write_state<S>(&self, state: S) 
    where
        S: Serialize
    {
        let state_string = serde_json::to_string(&state).unwrap();
        tokio::fs::write(STATE_PATH, state_string).await.unwrap();
    }

    pub async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) {
        let mut state = self.read_state().await;
        state
            .encryption_keys
            .insert(principal_address.to_lowercase(), encryption_key.to_string());

        self.write_state(state).await;
    }

    // check what happens if principal_address does not exist
    pub async fn get_encryption_key(&self, principal_address: &str) -> Option<String> {
        let state = self.read_state().await;
        let encryption_key = state.encryption_keys.get(principal_address)?;

        return Some(encryption_key.to_string());
    }
}

#[cfg(test)]
use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;

#[cfg(test)]
#[async_trait]
pub trait St {
    async fn create_state(&self);
    async fn read_state(&self) -> State;
    async fn write_state<S>(&self, state: S) where S: std::marker::Send + 'static;
    async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str);
    async fn get_encryption_key(&self, principal_address: &str) -> Option<String>;
}

#[cfg(test)]
mock! {
    pub StateService{}
    #[async_trait]
    impl St for StateService {
        async fn create_state(&self);
        async fn read_state(&self) -> State;
        async fn write_state<S>(&self, state: S) where S: std::marker::Send + 'static;
        async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str);
        async fn get_encryption_key(&self, principal_address: &str) -> Option<String>;
    }
}