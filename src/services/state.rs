use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STATE_PATH: &str = "./var/state.json";

#[async_trait]
pub trait StService {
    async fn get_encryption_key(&self, hash: &str) -> Option<String>;
    async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str);
}
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
        return Self {};
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
        S: Serialize,
    {
        let state_string = serde_json::to_string(&state).unwrap();
        tokio::fs::write(STATE_PATH, state_string).await.unwrap();
    }

}

#[async_trait]
impl StService for StateService {

    async fn get_encryption_key(&self, principal_address: &str) -> Option<String> {
        let state = self.read_state().await;
        let encryption_key = state.encryption_keys.get(principal_address)?;

        return Some(encryption_key.to_string());
    }

    async fn save_encryption_key(&self, principal_address: &str, encryption_key: &str) {
        let mut state = self.read_state().await;
        state
            .encryption_keys
            .insert(principal_address.to_lowercase(), encryption_key.to_string());

        self.write_state(state).await;
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
            expectations: HashMap::new()
        }
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
    async fn get_encryption_key(&self, _principal_address: &str) -> Option<String> {
        let expectation = self
            .expectations
            .get("get_encryption_key")
            .unwrap()
            .downcast_ref::<Expectation>()
            .unwrap();
        let result = (expectation.func.as_ref().unwrap())();

        return result;
    }

    async fn save_encryption_key(&self, _principal_address: &str, _encryption_key: &str) {}
}
