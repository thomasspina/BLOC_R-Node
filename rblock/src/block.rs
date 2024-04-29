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
    pub fn confirm_transactions(&self) -> bool {
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
    pub fn confirm_hash(&self) -> bool {
        self.get_hash() == hash(self.get_message())
    }

    /// verifies on the block if the difficulty and hash match
    pub fn confirm_difficulty(&self) -> bool {
        Block::verify_difficulty(self.get_hash(), self.get_difficulty())
    }

    /// returns the difficulty that a provided block should have.
    /// 
    /// difficulty works like this: a u32 is set as FFFFFFFF
    /// -> each 4 bit chunk of that u32 is compared each of the last 8 4-bit chunks
    ///     of the hash, an F in the difficulty means that the value of the respective 
    ///     4-bit chuck in the hash needs to take a value between 0 and F, an E between 0 and E,
    ///     a D between 0 and D, and so forth until its down to just zero.

    ///     the difficulty is adjusted by slowly subtracting one the each 4-bit chunk of the difficulty u32
    ///     until they are all 0
    /// 
    /// # Arguments
    /// * `base_block` - A &Block which specifies a reference to the block from which you want to know the difficulty
    /// * `comp_block` - A &Block which specifies a reference to the block for which you want to know the correct difficulty
    /// 
    /// # Returns
    /// A u32 which is the supposed difficulty of comp_block as a u32.
    /// 
    pub fn get_supposed_difficulty(base_block: &Block, comp_block: &Block) -> u32 {
        let latest_difficulty: u32 = base_block.get_difficulty();
        // get time difference between blocks
        let time_diff: u64 = comp_block.get_timestamp() - base_block.get_timestamp();

        let mut difficulty: u32 = latest_difficulty;

        if time_diff > super::BLOCK_SPEED {
            // reduce difficulty by increasing range of values per 4bit chuck
            for i in (0..=28).rev().step_by(4) {
                let mut bits: u32 = (latest_difficulty >> i) & 0xf;

                // if current 4 bits and next 4 bits are 1111
                if bits == 0xf { 
                    continue;
                }

                // add one to the 4 bit block
                bits += 1;

                let mask: u32 = 0xffffffff & !(0xf << i); // use a mask to eliminate 4 bits that are changed
                difficulty = (difficulty & mask) | (bits << i);
                break;
            }
        } else {
            // increase difficulty by reducing range of values per 4 bit chunk
            for i in (0..=28).step_by(4) {
                let mut bits: u32 = (latest_difficulty >> i) & 0xf;

                if bits == 0 { 
                    continue;
                }
                // sub one to the 4 bit block
                bits -= 1;
                
                let mask: u32 = 0xffffffff & !(0xf << i); // use a mask to eliminate 4 bits that are changed
                difficulty = (difficulty & mask) | (bits << i);
                break;
            }
        }


        difficulty
    }
}