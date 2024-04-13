use std::{cmp::max, fs};
use super::{Block, BLOCK_SPEED};

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
    pub fn get_new_block_difficulty(blockchain: &Blockchain, comp_block: &Block) -> u32 {
        // if not enough blocks in the blockchain, return latest difficulty
        let latest_diff: u32 = blockchain.get_latest_block().get_difficulty();
        
        let block: &Block = blockchain.get_latest_block();
        // get the difference between each block
        let diff: u64 = comp_block.get_timestamp() - block.get_timestamp();
        
        // init new diff
        let mut new_diff: u32 = latest_diff;

        // compare mean to desired speed
        if diff >= BLOCK_SPEED {
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
        let supposed_difficulty: u32 = Blockchain::get_new_block_difficulty(&self, &new_block);

        if !new_block.verify_transactions() {
            // error messages are already in the block method
            return;
        }

        if latest.get_hash() != new_block.get_prev_hash() {
            eprintln!("The new block is not linked to the previous block");

        } else if !new_block.verify_hash() {
            eprintln!("The new block's hash and its data do not fit");

        } else if new_block.get_difficulty() != supposed_difficulty {
            eprintln!("The new block's difficulty rating is supposed to be of {}", supposed_difficulty);
 
        } else if !Block::verify_difficulty(new_block.get_hash(), supposed_difficulty) {
            eprintln!("The new block's hash does not fit with the difficulty rating of {}", supposed_difficulty);

        } else {
            self.chain.push(new_block);
        }
    }

    pub fn store_blockchain(&self) {
        for block in &self.chain {
            block.store_block();
        }
    }


    /*
        returns a usable blockchain from the n latest blocks in memory
    */
    pub fn get_blockchain_from_files(n: Option<u8>) -> Option<Self> { // n is the number of blocks you want loaded in memory at once
        let n: u8 = n.unwrap_or(25);
        let mut max_height: u64 = 0;

        // get largest file number in memory
        if let Ok(files) = fs::read_dir("blocks_data") {
            for file in files.filter_map(Result::ok) {
                if let Some(name) = file.path().file_name() {
                    if let Some(name_str) = name.to_str() {
                        // -5 for .json tail of every file
                        max_height = max(name_str[..name_str.len()-5].parse().unwrap(), max_height);
                    }
                }
            }
        }   
        
        // verify that starting block exists theoretically
        if max_height < n as u64 {
            eprintln!("Starting block is smaller than genesis!");
            return None;
        }

        let mut blockchain: Blockchain = Blockchain {
            chain: vec![]
        };

        // add initial block
        match Block::get_block_from_file(max_height - n as u64) {
            Some(block) => {
                // initial checks on first block
                if block.verify_transactions() && block.verify_hash() && Block::verify_difficulty(block.get_hash(), block.get_difficulty()) {
                    blockchain.chain.push(block);
                } else {
                    eprintln!("Starting block is invalid.");
                    return None;
                }
            },
            None => {
                eprintln!("Starting block could not be found.");
                return None;
                // TODO: make a network call to see who has that block
            }
        };

        // repeat above match for every other block to load into memory
        for i in (max_height - n as u64 + 1)..max_height + 1 {
            match Block::get_block_from_file(i) {
                Some(block) => {
                    blockchain.add_block(block);
                },
                None => {
                    eprintln!("block of height {} could not be found.", i);
                    return None;
                    // TODO: make a network call to see who has that block
                }
            };
        }

        Some(blockchain)
    }

}