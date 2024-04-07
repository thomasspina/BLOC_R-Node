use sha256::hash;
use super::functions;
use super::Transaction;

#[derive(Clone)]
pub struct Block {
    height: u64,
    hash: String,
    timestamp: u64,
    prev_hash: String,
    nonce: u32,
    difficulty: u8, // number of tailing zeros
    merkle_root: String,
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new_genesis() -> Self {
        Block {
            height: 0,
            hash: "".to_owned(),
            timestamp: functions::get_unix_time(),
            nonce: 0, 
            difficulty: 0, 
            prev_hash: "".to_owned(),
            merkle_root: "".to_owned(),
            transactions: vec![]
        }
    }

    // returns None if one transaction is invalid
    pub fn new(prev_block: &Block, transactions: &Vec<Transaction>) -> Option<Self> {
        for transaction in transactions {
            if !transaction.verify() {
                return None;
            }
        }

        let mut new_block: Block = Block {
            height: prev_block.height + 1,
            hash: String::from(""), // hash needs to be set after block creation
            timestamp: functions::get_unix_time(),
            nonce: 0,
            difficulty: prev_block.difficulty,
            prev_hash: prev_block.hash.clone(),
            merkle_root: functions::get_merkel_root(transactions),
            transactions: transactions.to_owned()
        };

        new_block.set_hash();

        Some(new_block)
    }

    // used in case the difficulty has changed since the previous block
    pub fn set_difficulty(&mut self, diff: u8) {
        self.difficulty = diff;
    }

    pub fn incr_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }
    
    pub fn get_prev_hash(&self) -> &str {
        &self.prev_hash
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_difficulty(&self) -> &u8 {
        &self.difficulty
    }

    pub fn set_hash(&mut self) {
        self.hash = hash(self.get_message());
    }

    pub fn get_message(&self) -> String {
        format!("{}{}{}{}{}{}", 
                self.height, 
                self.timestamp,
                self.prev_hash,
                self.nonce,
                self.difficulty,
                self.merkle_root)
    }
}