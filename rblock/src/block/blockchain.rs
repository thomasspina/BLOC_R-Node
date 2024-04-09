use ecdsa::secp256k1::Point;
use sha256::hash;
use super::{Block, BLOCK_SPEED, MEAN_BLOCK_COUNT};

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block: Block = Block::new_genesis();

        Blockchain {
            chain: vec![genesis_block]
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn get_difficulty(&self) -> u8 {
        let latest_diff: u8 = self.get_latest_block().get_difficulty();
        if self.chain.len() < MEAN_BLOCK_COUNT as usize {
            return latest_diff;
        }
        
        let blocks: &[Block] = &self.chain[self.chain.len() - MEAN_BLOCK_COUNT as usize..];
        // get the difference between each block
        let diffs: Vec<u64> = blocks.iter()
                            .zip(blocks.iter().skip(1))
                            .map(|(b1, b2)| {
                                let t1 = b1.get_timestamp();
                                let t2 = b2.get_timestamp();
                                t2 - t1
                            })
                            .collect();
        
        // compare mean to desired speed
        if (diffs.iter().sum::<u64>() / (MEAN_BLOCK_COUNT - 1) as u64) >= BLOCK_SPEED {
            if latest_diff > 0 { latest_diff - 1 } else { 0 }
        } else {
            latest_diff + 1
        }
    }

    pub fn add_block(&mut self, new_block: Block) {
        let latest: &Block = self.get_latest_block();

        for transaction in new_block.get_transactions() {
            // Point::identity is miner reward
            if transaction.get_sender() != Point::identity() && !transaction.verify() {
                eprintln!("Cannot add block, a transaction is invalid");
                return;
            }
        }

        let new_hash: String = new_block.get_hash();

        if latest.get_hash() != new_block.get_prev_hash() {
            eprintln!("The new block is not linked to the previous block");

        } else if new_hash != hash(new_block.get_message()) {
            eprintln!("The new block's hash and its data do not fit");

        } else if new_block.get_difficulty() != self.get_difficulty() {
            eprintln!("The new block's difficulty rating is supposed to be of {}", self.get_difficulty());
 
        } else if new_hash[new_hash.len() - new_block.get_difficulty() as usize..] != *"0".repeat(new_block.get_difficulty().into()) {
            eprintln!("The new block's hash does not fit with the difficulty rating of {}", self.get_difficulty());

        } else {
            self.chain.push(new_block);

        }
    }
}