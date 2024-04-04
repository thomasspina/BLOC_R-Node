use super::Transaction;

struct Block {
    height: u64,
    hash: u64,
    version: String,
    timestamp: u128,
    nonce: u32,
    prev_hash: String,
    merkle_root: String,
    transactions: Vec<Transaction>
}