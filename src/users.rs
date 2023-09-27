use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    minSpendable: u64,
    maxSpendable: u64,
    pub address: String,
    pub macaroon: String,
    pub cert: String,
}

pub fn load_users() -> anyhow::Result<Vec<User>> {
    let data = std::fs::read_to_string("../benlnurl.json")?;

    let users = serde_json::from_str::<Vec<User>>(&data)?;

    Ok(users)
}
