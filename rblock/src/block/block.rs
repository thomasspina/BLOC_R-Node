use super::functions;
use super::Transaction;

#[derive(Clone)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
    pub prev_hash: String,
    pub merkle_root: String,
    pub transactions: Vec<Transaction>
}

// TODO: add difficulty changing capabilities
// TODO: add compressed blocks (no transactions)

impl Block {
    pub fn new(prev_block: &Block, 
            transactions: &Vec<Transaction>) -> Self {
        
        Block {
            height: prev_block.height + 1,
            hash: String::from(""), // hash needs to be set after block creation
            timestamp: functions::get_unix_time(),
            prev_hash: prev_block.hash.clone(),
            merkle_root: functions::get_merkel_root(transactions),
            transactions: transactions.to_owned()
        }
    }

    pub fn set_hash(&mut self) {
        self.hash = self.get_message();
    }

    pub fn get_message(&self) -> String {
        format!("{}{}{}{}", 
                self.height, 
                self.timestamp,
                self.prev_hash,
                self.merkle_root)
    }
}