use super::{Block, BLOCK_SPEED};

/// A blockchain is a chain of blocks, each block is linked to the previous block by its hash
pub struct Blockchain {
    /// the chain of blocks
    chain: Vec<Block>
}

impl Blockchain {
    /// creates a new blockchain with a genesis block
    pub fn new() -> Self {
        let genesis_block: Block = Block::new_genesis();

        Blockchain {
            chain: vec![genesis_block]
        }
    }

    /// returns the latest block in the blockchain
    /// 
    /// # Returns
    /// * A reference to the latest block
    /// 
    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    /// returns the difficulty after having adjusted it
    /// difficulty works like this: a u32 is set as FFFFFFFF
    /// -> each 4 bit chunk of that u32 is compared each of the last 8 4-bit chunks
    ///     of the hash, an F in the difficulty means that the value of the respective 
    ///     4-bit chuck in the hash needs to take a value between 0 and F, an E between 0 and E,
    ///     a D between 0 and D, and so forth until its down to just zero.

    ///     the difficulty is adjusted by slowly subtracting one the each 4-bit chunk of the difficulty u32
    ///     until they are all 0
    /// 
    /// # Arguments
    /// * `blockchain` - A reference to the blockchain
    /// * `comp_block` - A reference to the block to compare to
    /// 
    /// # Returns
    /// * The new difficulty as a u32
    /// 
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

    /// adds a new block to the blockchain
    /// makes block verification and adds it to the chain if it passes
    /// 
    /// # Arguments
    /// * `new_block` - The block to add
    /// 
    /// # Modifications
    /// * Adds the block to the chain if it passes verification, thus self needs to be mut
    /// 
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
}