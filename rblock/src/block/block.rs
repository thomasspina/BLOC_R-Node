use std::{collections::VecDeque, time::{Duration, SystemTime, UNIX_EPOCH}};
use sha256::hash;

use super::Transaction;

#[derive(Clone)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub version: u32,
    pub timestamp: u64,
    pub difficulty: u32,
    pub nonce: u32,
    pub prev_hash: String,
    pub merkle_root: String,
    pub transactions: Vec<Transaction>
}

impl Block {
    pub fn new(prev_block: &Block, 
            transactions: &Vec<Transaction>, 
            version: Option<u32>, 
            difficulty: Option<u32>, 
            nonce: Option<u32>) -> Block {
        
        Block {
            height: prev_block.height + 1,
            hash: String::from(""),
            version: version.unwrap_or(prev_block.version),
            timestamp: get_unix_time(),
            difficulty: difficulty.unwrap_or(prev_block.difficulty),
            nonce: nonce.unwrap_or_default(),
            prev_hash: prev_block.hash.clone(),
            merkle_root: get_merkel_root(transactions),
            transactions: transactions.to_owned()
        }
    }

    fn set_nonce(&mut self, nonce: u32) {
        self.nonce = nonce; 
    }

    fn calculate_hash(&mut self) {
        self.hash = hash(self.get_message())
    }   

    fn get_message(&self) -> String {
        format!("{}{}{}{}{}{}{}", 
                self.height, 
                self.version, 
                self.timestamp, 
                self.difficulty,
                self.nonce,
                self.prev_hash,
                self.merkle_root)
    }
}

fn get_unix_time() -> u64 {
    let current_time: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = current_time.duration_since(UNIX_EPOCH).unwrap();

    duration_since_epoch.as_secs()
}

fn get_merkel_root(transactions: &Vec<Transaction>) -> String {
    let mut nodes: VecDeque<String> = VecDeque::new();
    for transaction in transactions {
        nodes.push_back(transaction.get_hash());
    }

    // duplicate last element if odd number of leaves
    if nodes.len() % 2 == 1 {
        nodes.push_back(nodes[nodes.len() - 1].clone()); 
    }

    while nodes.len() > 1 {
        let f: String = nodes.pop_front().unwrap_or_default();
        let s: String = nodes.pop_front().unwrap_or_default();

        nodes.push_back(hash(f + &s));
    }

    nodes[0].clone()
}