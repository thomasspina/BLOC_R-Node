use crate::merkel::MerkelTree;
use super::Transaction;

pub struct Block {
    pub height: u64,
    pub hash: String,
    pub version: u32,
    pub timestamp: u32,
    pub difficulty: u32,
    pub nonce: u32,
    pub prev_hash: String,
    pub merkle_root: String,
    pub transactions: Vec<Transaction>
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) {
        Block::get_merkle_root(transactions);
    }

    fn get_merkle_root(transactions: Vec<Transaction>) -> String {
        let mut transaction_hashes: Vec<String> = vec![];
        for transaction in transactions {
            transaction_hashes.push(transaction.get_hash());
        }

        let merkel_tree: MerkelTree = MerkelTree::new(transaction_hashes);
        merkel_tree.root
    }
}