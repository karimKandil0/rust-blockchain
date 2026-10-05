use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub prev_hash: String,
    pub data: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now();
        let hash = Self::calculate_hash(index, &timestamp, &prev_hash, &data);
        Block {
            index,
            timestamp,
            prev_hash,
            data,
            hash,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: &DateTime<Utc>,
        prev_hash: &str,
        data: &str,
    ) -> String {
        let input = format!("{index}{timestamp}{prev_hash}{data}");
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}
