use core::fmt;
use ecdsa::secp256k1::Point;
use sha256::hash;
use super::functions;
use super::Transaction;
use super::REWARD;

#[derive(Clone)]
pub struct Block {
    height: u64,
    hash: String,
    timestamp: u64,
    prev_hash: String,
    nonce: u32,
    difficulty: u8, // number of tailing zeros
    merkel_root: String,
    transactions: Vec<Transaction>
}

// adds to_string for Block struct
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\theight: {}\n\thash: {}\n\ttimestamp: {}\n\tprev_hash: {}\n\tnonce: {}\n\tdifficulty: {}\n\tmerkel root: {}", 
            self.height, 
            self.hash,
            self.timestamp,
            self.prev_hash,
            self.nonce,
            self.difficulty,
            self.merkel_root)
    }
}

impl Block {
    pub fn new_genesis() -> Self {
        let mut genesis: Block = Block {
            height: 0,
            hash: "".to_owned(),
            timestamp: functions::get_unix_time(),
            nonce: 0, 
            difficulty: 0, 
            prev_hash: "".to_owned(),
            merkel_root: "".to_owned(),
            transactions: vec![]
        };

        genesis.set_hash();
        
        genesis
    }

    pub fn new(prev_block: &Block, transactions: &Vec<Transaction>) -> Self {
        let mut new_block: Block = Block {
            height: prev_block.height + 1,
            hash: String::from(""), // hash needs to be set after block creation
            timestamp: functions::get_unix_time(),
            nonce: 0,
            difficulty: prev_block.difficulty,
            prev_hash: prev_block.hash.clone(),
            merkel_root: functions::get_merkel_root(transactions),
            transactions: transactions.to_owned()
        };

        new_block.set_hash();

        new_block
    }

    // reward miner only if another reward doesn't already exist
    pub fn reward_miner(&mut self, miner_address: &Point) {
        for transaction in &self.transactions {
            if transaction.get_sender() == Point::identity() {
                eprintln!("There is already a reward in this block.");
                return;
            }
        }
        
        let reward_transaction: Transaction = Transaction::reward_transaction(miner_address, REWARD);
        
        self.transactions.push(reward_transaction);
        self.merkel_root = functions::get_merkel_root(&self.transactions);
        self.set_hash();
    }

    // used in case the difficulty has changed since the previous block
    pub fn set_difficulty(&mut self, diff: u8) {
        self.difficulty = diff;
        self.set_hash();
    }

    pub fn increment_and_hash(&mut self) {
        if self.nonce == u32::MAX {
            eprintln!("Nonce is at max u32, consider changing transactions.");
            return;
        }

        self.nonce += 1;
        self.set_hash();
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    
    pub fn get_merkel_root(&self) -> String {
        self.merkel_root.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub fn get_difficulty(&self) -> u8 {
        self.difficulty.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.clone()
    }

    fn set_hash(&mut self) {
        self.hash = hash(self.get_message());
    }

    pub fn get_message(&self) -> String {
        format!("{}{}{}{}{}{}", 
                self.height, 
                self.timestamp,
                self.prev_hash,
                self.nonce,
                self.difficulty,
                self.merkel_root)
    }
}