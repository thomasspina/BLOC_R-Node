use sha256::hash;
use super::{functions, Block};

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis_block: Block = Block { 
                                    height: 0, 
                                    hash: "".to_owned(), 
                                    timestamp: functions::get_unix_time(),
                                    prev_hash: "".to_owned(),
                                    merkle_root: functions::get_merkel_root(&vec![]),
                                    transactions: vec![]
                                };
        genesis_block.set_hash();

        Blockchain {
            chain: vec![genesis_block]
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn add_block(&mut self, new_block: Block) {
        // verify if block valid
        let latest: &Block = self.get_latest_block();
        assert_eq!(latest.hash, new_block.prev_hash);
        assert_eq!(new_block.hash, hash(new_block.get_message()));

        // TODO: add difficulty rating hash check
        // TODO: add transactions verification?

        self.chain.push(new_block);
    }
}