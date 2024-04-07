use sha256::hash;
use super::Block;

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis_block: Block = Block::new_genesis();
        genesis_block.set_hash();

        Blockchain {
            chain: vec![genesis_block]
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    // TODO:
    pub fn get_difficulty(&self) -> u8 {
        
        return 0;
    }

    pub fn add_block(&mut self, new_block: Block) {
        let latest: &Block = self.get_latest_block();

        for transaction in new_block.get_transactions() {
            if !transaction.verify() {
                eprintln!("A transaction is invalid");
                return;
            }
        }

        let new_block_hash: &str = new_block.get_hash();

        if latest.get_hash() != new_block.get_prev_hash() {
            eprintln!("The new block is not linked to the previous block");
        } else if new_block_hash != hash(new_block.get_message()) {
            eprintln!("The new block's hash and its data do not fit");
        } else if *new_block.get_difficulty() != self.get_difficulty() {
            eprintln!("The new block's difficulty rating is supposed to be of {}", self.get_difficulty());
        } else if new_block_hash[(new_block_hash.len() - usize::from(*new_block.get_difficulty()))..] != *"0".repeat(self.get_difficulty().into()) {
            eprintln!("The new block's hash does not fit with the difficulty rating of {}", self.get_difficulty());
        } else {
            self.chain.push(new_block);
        }
    }
}