use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    prev_hash: String, 
    data: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now();
        let hash = Self::calculate_hash(index, &timestamp, &prev_hash, &data);
        Block { index, timestamp, prev_hash, data, hash}
    }

    fn calculate_hash(index: u64, timestamp: &DateTime<Utc>, prev_hash: &str, data: &str) -> String {
        let input = format!("{index}{timestamp}{prev_hash}{data}");
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}

fn main() {
    println!("Hello, world!");
}
