use ecdsa::secp256k1::Point;
use sha256::hash;
use super::{Block, BLOCK_SPEED, MEAN_BLOCK_COUNT};

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    /*
        creates a new blockchain with a genesis block
    */
    pub fn new() -> Self {
        let genesis_block: Block = Block::new_genesis();

        Blockchain {
            chain: vec![genesis_block]
        }
    }

    /*
        returns the latest block in the blockchain
    */
    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    /*
        returns the difficulty after having adjusted it
        difficulty works like this: a u32 is set as FFFFFFFF
        -> each 4 bit chunk of that u32 is compared each of the last 8 4-bit chunks
            of the hash, an F in the difficulty means that the value of the respective 
            4-bit chuck in the hash needs to take a value between 0 and F, an E between 0 and E,
            a D between 0 and D, and so forth until its down to just zero.

            the difficulty is adjusted by slowly subtracting one the each 4-bit chunk of the difficulty u32
            until they are all 0
    */
    pub fn get_difficulty(blockchain: &Blockchain) -> u32 {
        // if not enough blocks in the blockchain, return latest difficulty
        let latest_diff: u32 = blockchain.get_latest_block().get_difficulty();
        if blockchain.chain.len() < MEAN_BLOCK_COUNT as usize {
            return latest_diff;
        }
        
        let blocks: &[Block] = &blockchain.chain[blockchain.chain.len() - MEAN_BLOCK_COUNT as usize..];
        // get the difference between each block
        let diffs: Vec<u64> = blocks.iter()
                            .zip(blocks.iter().skip(1))
                            .map(|(b1, b2)| {
                                let t1 = b1.get_timestamp();
                                let t2 = b2.get_timestamp();
                                t2 - t1
                            })
                            .collect();
        
        // init new diff
        let mut new_diff: u32 = latest_diff;

        // compare mean to desired speed
        if (diffs.iter().sum::<u64>() / (MEAN_BLOCK_COUNT - 1) as u64) >= BLOCK_SPEED {
            // reduce difficulty by increasing range of values per 4bit chuck
            for i in (0..=28).rev().step_by(4) {
                let mut bits: u32 = (latest_diff >> i) & 0xf;

                // if current 4 bits and next 4 bits are 1111
                if bits == 0xf { 
                    continue;
                }

                // add one to the 4 bit block
                bits += 1;

                let mask: u32 = 0xffffffff & !(0xf << i); // use a mask to eliminate 4 bits that are changed
                new_diff = (new_diff & mask) | (bits << i);
                break;
            }
        } else {
            // increase difficulty by reducing range of values per 4 bit chunk
            for i in (0..=28).step_by(4) {
                let mut bits: u32 = (latest_diff >> i) & 0xf;

                if bits == 0 { 
                    continue;
                }
                // sub one to the 4 bit block
                bits -= 1;
                
                let mask: u32 = 0xffffffff & !(0xf << i); // use a mask to eliminate 4 bits that are changed
                new_diff = (new_diff & mask) | (bits << i);
                break;
            }
        }

        new_diff
    }

    /*
        makes block verifications before adding block to the blockchain
    */
    pub fn add_block(&mut self, new_block: Block) {
        let latest: &Block = self.get_latest_block();

        for transaction in new_block.get_transactions() {
            // Point::identity is miner reward
            if transaction.get_sender() != Point::identity() && !transaction.verify() {
                eprintln!("Cannot add block, a transaction is invalid");
                eprintln!("{}", transaction);
                return;
            }
        }

        let new_hash: String = new_block.get_hash();
        let supposed_difficulty: u32 = Blockchain::get_difficulty(&self);

        if latest.get_hash() != new_block.get_prev_hash() {
            eprintln!("The new block is not linked to the previous block");

        } else if new_hash != hash(new_block.get_message()) {
            eprintln!("The new block's hash and its data do not fit");

        } else if new_block.get_difficulty() != supposed_difficulty {
            eprintln!("The new block's difficulty rating is supposed to be of {}", supposed_difficulty);
 
        } else if !Blockchain::verify_difficulty(new_block.get_hash(), supposed_difficulty) {
            eprintln!("The new block's hash does not fit with the difficulty rating of {}", supposed_difficulty);

        } else {
            self.chain.push(new_block);

        }
    }

    /*
        verifies that the 4-bit sized chunks of the hash are within the correct value range
    */
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
}