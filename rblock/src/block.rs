use core::fmt;
use ecdsa::secp256k1::Point;
use sha256::hash;
use super::{functions, Transaction, TRANSACTION_LIMIT_PER_BLOCK};
use serde::{Serialize, Deserialize};

/// A block in the blockchain
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    /// The height of the block, how many blocks is it above genesis
    height: u64,

    /// The hash of the block
    hash: String,

    /// The timestamp of the block
    timestamp: u64,

    /// The hash of the previous block
    prev_hash: String,

    /// The nonce of the block, used for hashing to comply with difficulty
    nonce: u32,

    /// The difficulty rating of the block
    difficulty: u32,
    
    /// The merkel root of the block
    merkel_root: String,

    /// The transactions in the block, limit is at 5000 transactions
    transactions: Vec<Transaction> 
}

/// adds display for Block struct for easy printing
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
    /// creates and returns new genesis block
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

    /// generates a new valid block who's transactions need to be verified and 
    /// who's hash needs to be rehashed to fit difficulty standard
    /// 
    /// # Arguments
    /// * `prev_block` - A reference to the previous block
    /// * `transactions` - A reference to a vector of transactions
    /// 
    /// # Returns
    /// * A new block
    /// 
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

    /// rewards miner only if another reward doesn't already exist
    /// pretty much obselete since you could just add it yourself when using 
    /// block::new in the transactions you pass
    /// 
    /// # Modifications
    /// * Adds a reward transaction to the block's transactions, hence the mut self
    /// 
    /// # Arguments
    /// * `miner_address` - A reference to the miner's public key
    /// 
    pub fn reward_miner(&mut self, miner_address: &Point) {
        // check if there is already a reward in the block
        for transaction in &self.transactions {
            if transaction.get_sender() == Point::identity() {
                eprintln!("There is already a reward in this block.");
                return;
            }
        }
        
        let reward_transaction: Transaction = Transaction::reward_transaction(miner_address);
        
        self.transactions.push(reward_transaction);
        self.merkel_root = functions::get_merkel_root(&self.transactions);
        self.set_hash();
    }

    /// sets the block's difficulty
    /// used in case the difficulty has changed since the previous block
    /// 
    /// # Arguments
    /// * `diff` - The new difficulty rating
    /// 
    /// # Modifications
    /// * Changes the block's difficulty rating, hence the mut self
    /// 
    pub fn set_difficulty(&mut self, diff: u32) {
        self.difficulty = diff;
        self.set_hash();
    }

    /// increments nonce and generates hash
    /// 
    /// # Modifications
    /// * Changes the block's nonce and hash, hence the mut self
    /// 
    pub fn increment_and_hash(&mut self) {
        // if nonce is at max, then the block is invalid
        if self.nonce == u32::MAX {
            eprintln!("Nonce is at max u32, consider changing transactions.");
            return;
        }

        self.nonce += 1;
        self.set_hash();
    }

    /// returns the current block's hash
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    
    /// returns the current block's merkel root
    pub fn get_merkel_root(&self) -> String {
        self.merkel_root.clone()
    }

    /// returns the current block's previous hash
    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }

    /// returns the current block's transactions
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    /// returns the current block's difficulty
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty.clone()
    }

    /// returns the current block's timestamp
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.clone()
    }

    /// returns the current block's height
    pub fn get_height(&self) -> u64 {
        self.height.clone()
    }
 
    /// Hashes with the data in the block and sets the hash 
    /// 
    /// # Modifications
    /// * Changes the block's hash, hence the mut self
    /// 
    fn set_hash(&mut self) {
        self.hash = hash(self.get_message());
    }

    /// gets the message that was used to hash the block
    pub fn get_message(&self) -> String {
        format!("{}{}{}{}{}{}", 
                self.height, 
                self.timestamp,
                self.prev_hash,
                self.nonce,
                self.difficulty,
                self.merkel_root)
    }

    /// verifies that the 4-bit sized chunks of the hash are within the correct value range
    /// 
    /// # Arguments
    /// * `hash` - The hash to verify
    /// * `difficulty` - The difficulty rating to compare the hash to
    /// 
    /// # Returns
    /// * True if the hash is within the difficulty rating, false otherwise
    /// 
    pub fn verify_difficulty(hash: String, difficulty: u32) -> bool {

        // get last 8 characters (4 bytes) of the hash to compare for difficulty rating
        let hash_u32: u32 = u32::from_str_radix(&hash[hash.len() - 8..], 16).unwrap();

        // half-byte per half-byte comparison
        for i in (0..=28).step_by(4) {
            let difficulty_bits: u32 = (difficulty >> i) & 0xf;
            let hash_bits: u32 = (hash_u32 >> i) & 0xf;

            if hash_bits > difficulty_bits {
                return false;
            }
        }

        true
    }

    /// checks every transaction to make sure  that its good
    /// 
    /// # Returns
    /// * True if all transactions are valid, false otherwise
    /// 
    pub fn verify_transactions(&self) -> bool {
        // too many transactions
        if self.transactions.len() > TRANSACTION_LIMIT_PER_BLOCK {
            eprintln!("{} is too many transactions", self.transactions.len());
            return false;
        }

        for transaction in &self.transactions {
            // Point::identity is miner reward sender
            if transaction.get_sender() != Point::identity() && !transaction.verify() {
                eprintln!("A transaction is invalid");
                eprintln!("{}", transaction);
                return false;
            }
        }

        return true;
    }

    /// verifies if the hash of the block fits with current data
    /// 
    /// # Returns
    /// * True if the hash is correct, false otherwise
    pub fn verify_hash(&self) -> bool {
        self.get_hash() == hash(self.get_message())
    }

    /// verifies on the block if the difficulty and hash match
    pub fn confirm_difficulty(&self) -> bool {
        Block::verify_difficulty(self.get_hash(), self.get_difficulty())
    }
}