use super::Transaction;

pub struct Block {
    pub height: u64,
    pub hash: u64,
    pub version: String,
    pub timestamp: u128,
    pub nonce: u32,
    pub prev_hash: String,
    pub merkle_root: String,
    pub transactions: Vec<Transaction>
}