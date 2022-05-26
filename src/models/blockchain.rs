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
            HashingPayload::from_block_for_next_block(&latest_block, String::from("data"));
        let hash = calculate_hash(hashing_payload);
        println!("{}", hash);

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

fn calculate_hash(payload: HashingPayload) -> String {
    let payload_string = format!(
        "{}{}{}{}",
        payload.index,
        payload.parent_hash.unwrap_or("".to_string()),
        payload.timestamp,
        payload.data
    );

    let hash_array = Sha256::digest(payload_string);
    let hash = format!("{:x}", hash_array);

    return hash;
}
