use super::block::Block;

use std::time::{SystemTime, UNIX_EPOCH};

use mongodb::Client;
use sha2::{Digest, Sha256};

pub struct Blockchain {
    context: Client,
}

impl Blockchain {
    pub fn new(client: &Client) -> Blockchain {
        Blockchain {
            context: client.clone(),
        }
    }
}

impl Blockchain {
    pub async fn blocks(&self) -> Result<Vec<Block>, &'static str> {
        Block::get_all(&self.context).await
    }

    pub async fn get_latest_block(&self) -> Result<Block, &'static str> {
        Block::get_last(&self.context).await
    }

    pub async fn generate_next_block(&self, data: String) -> Result<(), &'static str> {
        let latest_block = match Block::get_last(&self.context).await {
            Err(error) => return Err(error),
            Ok(value) => value,
        };

        let payload = HashingPayload::from_block_for_next_block(&latest_block, data);
        let hash = calculate_hash(&payload);
        let next_block = Block {
            index: payload.index,
            hash,
            parent_hash: payload.parent_hash,
            timestamp: payload.timestamp,
            data: payload.data,
        };

        match self.add_to_chain(&next_block, &latest_block).await {
            Err(error) => return Err(error),
            Ok(_) => (),
        };

        return Ok(());
    }

    pub async fn add_to_chain_from_request(
        &self,
        request_body: &String,
    ) -> Result<(), &'static str> {
        let next_block: Block = match serde_json::from_str(request_body) {
            Err(error) => {
                println!("error: {:?}", error);
                return Err("invalid payload");
            }
            Ok(value) => value,
        };

        let latest_block = match Block::get_last(&self.context).await {
            Err(error) => return Err(error),
            Ok(value) => value,
        };

        match self.add_to_chain(&next_block, &latest_block).await {
            Err(error) => return Err(error),
            Ok(_) => (),
        };

        Ok(())
    }

    pub async fn replace_chain_from_request(
        &self,
        request_body: &String,
    ) -> Result<(), &'static str> {
        let new_chain: Vec<Block> = match serde_json::from_str(request_body) {
            Err(error) => {
                println!("error: {:?}", error);
                return Err("invalid payload");
            }
            Ok(value) => value,
        };

        let current_chain = match self.blocks().await {
            Err(error) => return Err(error),
            Ok(value) => value,
        };

        if !self.validate_chain(&new_chain, &current_chain) {
            return Err("invalid chain provided");
        }

        // Flush database and replace database with current chain

        Ok(())
    }
}

impl Blockchain {
    async fn add_to_chain(
        &self,
        next_block: &Block,
        latest_block: &Block,
    ) -> Result<(), &'static str> {
        let next_block_is_valid = self.validate_next_block(&next_block, &latest_block);
        if !next_block_is_valid {
            return Err("invalid new block");
        }

        match next_block.insert(&self.context).await {
            Err(error) => return Err(error),
            Ok(value) => value,
        };

        return Ok(());
    }

    fn validate_chain(&self, new_chain: &Vec<Block>, current_chain: &Vec<Block>) -> bool {
        if new_chain.len() <= current_chain.len() {
            return false;
        }

        let genesis_block = Block::genesis_block();

        if genesis_block != new_chain[0] {
            return false;
        }

        for index in 1..new_chain.len() {
            let next_block = &new_chain[index];
            let previous_block = &new_chain[index - 1];
            if !self.validate_next_block(next_block, previous_block) {
                return false;
            }
        }

        false
    }

    fn validate_next_block(&self, next_block: &Block, latest_block: &Block) -> bool {
        next_block.index == latest_block.index + 1
            && next_block.parent_hash == Some(latest_block.hash.clone())
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
        payload
            .parent_hash
            .clone()
            .unwrap_or_else(|| "".to_string()),
        payload.timestamp,
        payload.data
    );

    let hash_array = Sha256::digest(payload_string);
    let hash = format!("{:x}", hash_array);

    return hash;
}
