use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    minSpendable: u64,
    maxSpendable: u64,
    pub address: String,
    pub macaroon: String,
    pub cert: String,
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
    let data = std::fs::read_to_string("/Users/ben/github.com/benlnurl/benlnurl.json")?;

    let users = serde_json::from_str::<HashMap<String, NodeInfo>>(&data)?;

    Ok(users)
}
