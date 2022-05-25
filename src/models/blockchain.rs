use super::block::Block;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block {
            index: 0,
            hash: String::from("cd2fb2ace926608315b2a5bd1bc2a259dce057a21ed63351adc0b1326da2a99e"),
            parent_hash: None,
            timestamp: 1652722519,
            data: String::from("The Genesis block!!!"),
        };

        Blockchain {
            blocks: vec![genesis_block],
        }
    }
}

impl Blockchain {
    pub fn get_latest_block(&self) -> Option<&Block> {
        let block = self.blocks.last().clone();
        block
    }

    pub fn generate_next_block(&self) -> Result<(), &'static str> {
        let latest_block = match self.get_latest_block() {
            None => return Err("could not get latest block"),
            Some(value) => value,
        };

        let hashing_payload =
            HashingPayload::from_block_for_new_block(&latest_block, String::from("data"));

        calculate_hash(hashing_payload);

        return Ok(());
    }
}

#[derive(Serialize, Deserialize)]
struct HashingPayload {
    index: u64,
    parent_hash: String,
    timestamp: u64,
    data: String,
}

impl HashingPayload {
    fn from_block_for_new_block(block: &Block, data: String) -> HashingPayload {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        HashingPayload {
            index: block.index + 1,
            parent_hash: block.hash.clone(),
            timestamp,
            data,
        }
    }
}

fn calculate_hash(payload: HashingPayload) -> Result<(), &'static str> {
    let payload = match serde_json::to_vec(&payload) {
        Err(_) => return Err("could not parse hashing payload"),
        Ok(value) => value,
    };

    let hash = Sha256::digest(payload).to_vec();

    return Ok(());
}
