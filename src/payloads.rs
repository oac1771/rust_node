use crate::services::models::Data;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BootStrap {
    pub contract_address: String,
}

#[derive(Deserialize)]
pub struct Register {
    pub data: Data,
    pub principal_address: String,
}

#[derive(Deserialize)]
pub struct Remove {
    pub principal_address: String,
    pub token_id: u128,
}
