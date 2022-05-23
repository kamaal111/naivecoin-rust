#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub previous_hash: Option<String>,
    pub timestamp: u64,
    pub data: String,
}
