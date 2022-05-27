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
    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn generate_next_block(&mut self, data: String) -> Result<(), &'static str> {
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

        match self.add_to_chain(next_block) {
            Err(error) => return Err(error),
            Ok(_) => (),
        };

        return Ok(());
    }

    fn add_to_chain(&mut self, next_block: Block) -> Result<(), &'static str> {
        let latest_block = self.blocks.last().unwrap();

        let next_block_is_valid = self.validate_next_block(&next_block, latest_block);
        if !next_block_is_valid {
            return Err("invalid new block");
        }

        self.blocks.push(next_block);

        return Ok(());
    }
}

impl Blockchain {
    fn validate_next_block(&self, next_block: &Block, previous_block: &Block) -> bool {
        next_block.index == previous_block.index + 1
            && next_block.parent_hash == Some(previous_block.hash.clone())
            && next_block.hash.clone() == calculate_hash(&HashingPayload::from_block(next_block))
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

    fn from_block(block: &Block) -> HashingPayload {
        HashingPayload {
            index: block.index,
            parent_hash: block.parent_hash.clone(),
            timestamp: block.timestamp,
            data: block.data.clone(),
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
