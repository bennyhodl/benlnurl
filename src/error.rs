use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BenlnurlError {
    pub status: String,
    pub reason: String,
}
