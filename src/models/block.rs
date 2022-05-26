use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub timestamp: u64,
    pub data: String,
}
