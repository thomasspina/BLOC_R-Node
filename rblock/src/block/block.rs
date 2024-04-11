use core::fmt;
use ecdsa::secp256k1::Point;
use sha256::hash;
use super::functions;
use super::Transaction;
use super::REWARD;

#[derive(Clone)]
pub struct Block {
    height: u64, // how many blocks is it above genesis
    hash: String,
    timestamp: u64,
    prev_hash: String,
    nonce: u32, // used for hashing to comply with difficulty
    difficulty: u32,
    merkel_root: String, // https://en.wikipedia.org/wiki/Merkle_tree
    transactions: Vec<Transaction> // TODO: add Option with relevant methods for further compression
}

/*
    adds to_string for Block struct
*/
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
    /*
        generates a new genesis block
    */
    pub fn new_genesis() -> Self {
        let mut genesis: Block = Block {
            height: 0,
            hash: "".to_owned(),
            timestamp: functions::get_unix_time(),
            nonce: 0, 
            difficulty: 0xffffffff, 
            prev_hash: "".to_owned(),
            merkel_root: "".to_owned(),
            transactions: vec![]
        };

        genesis.set_hash();
        
        genesis
    }

    /*
        generates a new valid block who's transactions need to be verified and 
        who's hash needs to be rehashed to fit difficulty standard
    */
    pub fn new(prev_block: &Block, transactions: &Vec<Transaction>) -> Self {
        let mut new_block: Block = Block {
            height: prev_block.height + 1,
            hash: String::from(""),
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

    /*
        rewards miner only if another reward doesn't already exist
    */
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

    /*
        sets the block's difficulty
        used in case the difficulty has changed since the previous block
    */
    pub fn set_difficulty(&mut self, diff: u32) {
        self.difficulty = diff;
        self.set_hash();
    }

    /*
        increments nonce and generates hash
    */
    pub fn increment_and_hash(&mut self) {
        if self.nonce == u32::MAX {
            eprintln!("Nonce is at max u32, consider changing transactions.");
            return;
        }

        self.nonce += 1;
        self.set_hash();
    }

    /*
        returns current block hash
    */
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    
    /*
        returns current block merkel root
    */
    pub fn get_merkel_root(&self) -> String {
        self.merkel_root.clone()
    }

    /*
        returns current block's prev block's hash
    */
    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }

    /*
        returns the current block's transactions
    */
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    /*
        returns current block's difficulty
    */
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty.clone()
    }


    /*
        returns current block's timestamp
    */
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.clone()
    }


    /*
        hashes current block's info and sets the current hash to that hash
    */
    fn set_hash(&mut self) {
        self.hash = hash(self.get_message());
    }

    /*
        gets the info that the block hashes
    */
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