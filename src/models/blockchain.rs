use super::block::Block;

use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block {
            index: 0,
            hash: "cd2fb2ace926608315b2a5bd1bc2a259dce057a21ed63351adc0b1326da2a99e".to_string(),
            parent_hash: None,
            timestamp: 1652722519,
            data: "The Genesis block!!!".to_string(),
        };

        Blockchain {
            blocks: vec![genesis_block],
        }
    }
}

impl Blockchain {
    pub fn generate_next_block(&self, data: String) -> Result<(), &'static str> {
        let latest_block = match self.blocks.last() {
            None => return Err("could not get latest block"),
            Some(value) => value,
        };

        let payload = HashingPayload::from_block_for_next_block(latest_block, data);
        let hash = calculate_hash(&payload);
        let next_block = Block {
            index: payload.index,
            hash,
            parent_hash: payload.parent_hash,
            timestamp: payload.timestamp,
            data: payload.data,
        };
        println!("{:#?}", next_block);

        return Ok(());
    }
}

struct HashingPayload {
    index: u64,
    parent_hash: Option<String>,
    timestamp: u64,
    data: String,
}

impl HashingPayload {
    fn from_block_for_next_block(block: &Block, data: String) -> HashingPayload {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        HashingPayload {
            index: block.index + 1,
            parent_hash: Some(block.hash.clone()),
            timestamp,
            data,
        }
    }
}

fn calculate_hash(payload: &HashingPayload) -> String {
    let payload_string = format!(
        "{}{}{}{}",
        payload.index,
        payload.parent_hash.clone().unwrap_or("".to_string()),
        payload.timestamp,
        payload.data
    );

    let hash_array = Sha256::digest(payload_string);
    let hash = format!("{:x}", hash_array);

    return hash;
}
