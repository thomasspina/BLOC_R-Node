use std::io::{self, ErrorKind};
use dirs::home_dir;
use ecdsa::secp256k1::Point;
use rblock::Block;
use rusty_leveldb::{Options, Status, DB};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub struct BlocksDB {
    db: DB
}

impl BlocksDB {
    /*
        creates a new db if one doesn't alread exist,
        and creates/opens a file in HOMEDIR/.r_blocks where the db will operate
    */
    pub fn start_db() -> Result<Self, Status> {
        let mut options: Options = Options::default();
        options.create_if_missing = true;
        
        let path = home_dir().ok_or_else(|| {
            io::Error::new(ErrorKind::NotFound, "Home directory could not be found")
        })?;

        let db: DB = DB::open(path.join(".r_blocks"), options)?;
        Ok(BlocksDB { db })
    }

    /*
        adds a block to the db
    */
    pub fn put_block(&mut self, block: &Block) {
        let index: [u8; 8] = block.get_height().to_le_bytes();

        match self.read_block(block.get_height()) {
            Some(_) => {
                eprintln!("Cannot add block to db as block already exists in db")
            },
            None => {
                // serialize block
                let binary: Vec<u8> = bincode::serialize(block).unwrap();
                self.db.put(&index, &binary).unwrap();

                if true { // TODO: replace true with condition that the block is the greatest in the db
                        // need to do this because otherwise chainstate is only dependent on what we have
                        // in db on machine
                    self.update_chainstate(block);
                }
            }
        }
    }

    /*
        updates the chainstate in the db with the transactions of a given block
    */
    fn update_chainstate(&mut self, block: &Block) {
        for transaction in block.get_transactions() {
            let sender: Point = transaction.get_sender();
            let recipient: Point = transaction.get_recipient();

            // subtract transaction value from sender balance
            let new_sender_balance: f32 = self.get_address_balance(&sender).unwrap() - transaction.get_amount();
            
            // add transaction value to recipient balance
            let new_recipient_balance: f32 = self.get_address_balance(&recipient).unwrap() + transaction.get_amount();

            // store both new values for the public keys
            let _ = self.db.put(&bincode::serialize(&sender).unwrap(), &new_sender_balance.to_le_bytes()).unwrap();
            let _ = self.db.put(&bincode::serialize(&recipient).unwrap(), &new_recipient_balance.to_le_bytes()).unwrap();
        }
    }

    fn reverse_chainstate(&mut self, block: &Block) {
        // TODO:
    }

    /*
        returns a certain addresse's balance in the db
    */
    pub fn get_address_balance(&mut self, public_key: &Point) -> Option<f32> {
        match self.db.get(&bincode::serialize(public_key).unwrap()) {
            Some(val) => {
                let mut reader = Cursor::new(val);
                Some(reader.read_f32::<LittleEndian>().unwrap())
            },
            None => None
        }
    }

    /*
        returns block at index height if it exists in db
    */
    pub fn read_block(&mut self, height: u64) -> Option<Block> {
        let bin = self.db.get(&height.to_le_bytes());
        match bin {
            Some(bin) => {
                let block: Block = bincode::deserialize(&bin).unwrap();
                Some(block)
            },
            None => {
                eprintln!("Block {} not found in db", height);
                None
            }
        }
    }

    /*
        TODO:
            get latest block function
            is blockchain from rblock still necessary?

        note to self:
            you're gonna have to keep chainstate up to date on the network
    */

    /*
        deletes the block at index height in db, used for pruning
    */
    pub fn delete_block(&mut self, height: u64) {
        // TODO: reverse all chainstate transactions if it's the biggest in the db
        match self.db.delete(&height.to_le_bytes()) {
            Err(e) => { eprintln!("{e}"); },
            _ => {}
        }
    }


}
