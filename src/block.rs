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
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            data,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();

        return block;
    }

    pub fn calculate_hash(&self) -> String {
        let input =
            serde_json::to_string(&(self.index, &self.timestamp, &self.prev_hash, &self.data))
                .unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}
