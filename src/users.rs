use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};
use std::env::current_dir;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    minSpendable: u64,
    maxSpendable: u64,
    tag: String,
    metadata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub address: String,
    pub macaroon: String,
    pub cert: String,
}

pub fn load_users() -> anyhow::Result<HashMap<String, NodeInfo>> {
    info!("Getting from db");
    println!("Getting from db");

    let dir = current_dir()?.join("benlnurl.json");
    let data = std::fs::read_to_string(dir)?;

    let users = serde_json::from_str::<HashMap<String, NodeInfo>>(&data)?;

    Ok(users)
}
