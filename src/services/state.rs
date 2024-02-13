use axum::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::models::StateServiceError;

const STATE_PATH: &str = "./var/state.json";

#[async_trait]
pub trait StService {
    async fn get_encryption_key(&self, hash: &str) -> Result<String, StateServiceError>;
    async fn save_encryption_key(
        &self,
        principal_address: &str,
        encryption_key: &str,
    ) -> Result<(), StateServiceError>;
}
pub struct StateService {}

#[derive(Deserialize, Serialize)]
pub struct State {
    encryption_keys: HashMap<String, String>,
    neighbors: HashMap<String, String>,
}

impl State {
    pub fn new() -> State {
        return State {
            encryption_keys: HashMap::new(),
            neighbors: HashMap::new(),
        };
    }
}

impl StateService {
    pub fn new() -> Self {
        return Self {};
    }
}

// change this to create_or_update method to make /bootstrap itempotent
pub async fn create_state() -> Result<(), StateServiceError> {
    let state = State::new();

    let parent_directories = STATE_PATH.split("state.json").collect::<Vec<&str>>();
    tokio::fs::create_dir_all(parent_directories[0]).await?;

    write_state(state).await?;
    return Ok(());
}

async fn read_state() -> Result<State, StateServiceError> {
    let state_bytes = tokio::fs::read(STATE_PATH).await?;

    let state: State = serde_json::from_slice(&state_bytes)?;

    return Ok(state);
}

async fn write_state<S>(state: S) -> Result<(), StateServiceError>
where
    S: Serialize,
{
    let state_string = serde_json::to_string(&state)?;
    tokio::fs::write(STATE_PATH, state_string).await?;
    return Ok(());
}

#[async_trait]
impl StService for StateService {
    async fn get_encryption_key(
        &self,
        principal_address: &str,
    ) -> Result<String, StateServiceError> {
        let state = read_state().await?;

        if let Some(key) = state.encryption_keys.get(principal_address) {
            return Ok(key.to_string());
        } else {
            return Err(StateServiceError::EncryptionKeyNotFound(format!(
                "Encryption key for {} not found",
                principal_address
            )));
        };
    }

    async fn save_encryption_key(
        &self,
        principal_address: &str,
        encryption_key: &str,
    ) -> Result<(), StateServiceError> {
        let mut state = read_state().await?;
        state
            .encryption_keys
            .insert(principal_address.to_lowercase(), encryption_key.to_string());

        write_state(state).await?;
        return Ok(());
    }
}

#[cfg(test)]
pub struct MockStateService {
    expectations: std::collections::HashMap<
        String,
        Box<dyn std::any::Any + std::marker::Sync + std::marker::Send>,
    >,
}

#[cfg(test)]
pub struct Expectation {
    pub func:
        Option<Box<dyn Fn() -> Option<String> + 'static + std::marker::Sync + std::marker::Send>>,
}

#[cfg(test)]
impl Expectation {
    pub fn returns(
        &mut self,
        func: impl Fn() -> Option<String> + 'static + std::marker::Sync + std::marker::Send,
    ) {
        self.func = Some(Box::new(func));
    }
}

#[cfg(test)]
impl MockStateService {
    pub fn new() -> MockStateService {
        return Self {
            expectations: HashMap::new(),
        };
    }

    pub fn expect_get_encryption_key(&mut self) -> &mut Expectation {
        self.expectations
            .entry("get_encryption_key".to_string())
            .or_insert_with(|| Box::new(Expectation { func: None }))
            .downcast_mut::<Expectation>()
            .unwrap()
    }
}

#[cfg(test)]
#[async_trait]
impl StService for MockStateService {
    async fn get_encryption_key(
        &self,
        _principal_address: &str,
    ) -> Result<String, StateServiceError> {
        let expectation = self
            .expectations
            .get("get_encryption_key")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())().unwrap();

        return Ok(result);
    }

    async fn save_encryption_key(
        &self,
        _principal_address: &str,
        _encryption_key: &str,
    ) -> Result<(), StateServiceError> {
        return Ok(());
    }
}
