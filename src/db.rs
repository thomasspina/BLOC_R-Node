use std::io::{self, ErrorKind};
use dirs::home_dir;
use ecdsa::secp256k1::Point;
use rblock::{Block, Transaction};
use reqwest::StatusCode;
use rusty_leveldb::{Options, Status, DB};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

const LATEST_BLOCK_KEY: &'static [u8; 6] = b"latest";
const DB_FILENAME: &'static str = ".r_blocks";


/// A struct that represents a database of blocks.
/// 
/// # Fields
/// * `db` - A DB object that represents the database of blocks
/// 
pub struct BlocksDB {
    db: DB
}

// TODO: what to put in the db if it's empty (always has genesis block though)

// TODO: function to wipe db to restart from 0 if db was corrutped
// TODO: function to get correct difficulty

impl BlocksDB {
    /// Starts the database and returns a BlocksDB object with the database
    /// 
    /// # Modifications
    /// This method creates a new database file in the home directory of the user.
    /// 
    pub fn start_db() -> Result<Self, Status> {
        let mut options: Options = Options::default();
        options.create_if_missing = true; // create DB if missing
        
        // get home directory
        let path = home_dir().ok_or_else(|| {
            io::Error::new(ErrorKind::NotFound, "Home directory could not be found")
        })?;

        let db: DB = DB::open(path.join(DB_FILENAME), options)?;
        Ok(BlocksDB { db })
    }

    /// Puts a block into the db if it doesn't already exist.
    /// 
    /// # Arguments
    /// * `block` - A &Block which specifies a reference to the block to put into the db
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling put on it.
    /// 
    pub fn put_block(&mut self, block: &Block) -> Result<(), Status> {
        match self.get_block(block.get_height()) {
            Ok(_) => {
                // block already exists
                Ok(())
            },
            Err(e) => {
                if e.code != rusty_leveldb::StatusCode::NotFound {
                    // serialize block
                    let binary: Vec<u8> = bincode::serialize(block).unwrap(); // blocks are always serializable
                    self.db.put(&block.get_height().to_le_bytes(), &binary)?;

                    // unwrap is safe since there's is always a latest block
                    let latest_block: Block = self.get_latest_block()?;
                    if latest_block.get_height() == block.get_height() - 1 {

                        // update db with new latest block info
                        self.update_latest_block(block)?;
                        self.update_chainstate(block.get_transactions())?;
                    }

                    ()
                }

                Err(e)
            }
        }
    }

    /// verifies that the transactions are valid and can be added to the chainstate.
    /// only verifies that the sender exists and has enough money to send the money in the transaction.
    /// 
    /// # Arguments
    /// * `transactions` - A Vec<Transaction> which specifies the transactions to verify
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling get on it.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(bool) if the transactions are valid, or an error if they are not.
    /// 
    pub fn verify_transactions(&mut self, transactions: &Vec<Transaction>) -> Result<(), Status> {
        for transaction in transactions {
            let sender: Point = transaction.get_sender();

            if sender != Point::identity() {
                // get original balances
                let sender_balance: Option<f32> = self.get_balance(&sender);

                // check if sender exists and has enough money to send
                if sender_balance.is_none() || (sender_balance.is_some() && sender_balance.unwrap() < transaction.get_amount()) {
                    // status code invalid data as data is most likely invalid
                    return Err(Status::new(rusty_leveldb::StatusCode::InvalidData, "Transaction information is not good for chainstate."));
                }
            }
        }

        Ok(())
    }


    pub fn get_latest_difficulty(&mut self) -> Result<u32, Status> {
        let latest_block: Block = self.get_latest_block()?;

        
    }

    /// Updates the chainstate with the transactions of a given block.
    /// Multiple checks should be made before using this method. Method is private so as to not invalidate the data in the db
    /// 
    /// # Arguments
    /// * `transactions` - A Vec<Transaction> which specifies the transactions to update the chainstate with
    /// 
    /// # Modifications
    /// This method changes often multiple addresses' balances using put on the db object.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the chainstate was successfully updated, or an error if it was not.
    ///
    fn update_chainstate(&mut self, transactions: Vec<Transaction>) -> Result<(), Status> {
        // verify that the transactions are valid according to the chainstate
        self.verify_transactions(&transactions)?; 

        for transaction in transactions {
            let mut recorded_transactions = vec![];
            let sender: Point = transaction.get_sender();
            let recipient: Point = transaction.get_recipient();

            // only do sender related operations if transaction isn't miner reward
            if sender != Point::identity() {

                // modified sender balance
                let mod_sender_balance: f32 = self.get_balance(&sender).unwrap() - transaction.get_amount();

                // update both modified balances, reverse changes if error in update
                match self.db.put(&bincode::serialize(&sender).unwrap(), &mod_sender_balance.to_le_bytes()) {
                    Ok(_) => {},
                    Err(e) => {
                        self.reverse_chainstate(recorded_transactions).expect("DB might have corrupt chainstate. Reset db"); // panic if cannot reverse transactions
                        return Err(e);
                    }
                }
            }

            let mod_recipient_balance: f32 = self.get_balance(&recipient).unwrap_or(0.0) + transaction.get_amount();
            
            // check that the db puts correctly
            match self.db.put(&bincode::serialize(&recipient).unwrap(), &mod_recipient_balance.to_le_bytes()) {
                Ok(_) => {},
                Err(e) => {
                    self.reverse_chainstate(recorded_transactions).expect("DB might have corrupt chainstate. Reset db"); // panic if cannot reverse transactions
                    return Err(e);
                }
            }

            recorded_transactions.push(transaction);
        }

        Ok(())
    }

    /// Updates the latest block in the db to the given block.
    /// 
    /// # Arguments
    /// * `block` - A &Block which specifies a reference to the block to update the latest block to
    /// 
    /// # Modifications
    /// This method changes the latest block in the db by calling put on the db object.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the block was successfully updated, or an error if it was not.
    /// 
    fn update_latest_block(&mut self, block: &Block) -> Result<(), Status> {
        self.db.put(LATEST_BLOCK_KEY, &bincode::serialize(block).unwrap())?;

        Ok(())
    }

    /// Reverts the chainstate with the transactions of a given block. 
    /// Multiple checks should be made before using this method.
    /// 
    /// Method is not safe as it doesn't check if the recipient has enough balance to revert the amount.
    /// It simply iterates through the transactions and once an error is encountered, it panics and stops without reverting changes.
    /// 
    /// # Arguments
    /// * `transactions` - A Vec<Transaction> which specifies the transactions to revert the chainstate with
    /// 
    /// # Modifications
    /// This method changes often multiple addresses' balances using put on the db object.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the chainstate was successfully reversed, or an error if it was not.
    ///
    fn reverse_chainstate(&mut self, transactions: Vec<Transaction>) -> Result<(), Status> {
        for transaction in transactions {
            let sender: Point = transaction.get_sender();
            let recipient: Point = transaction.get_recipient();

            let mod_sender_balance: f32 = self.get_balance(&sender).unwrap() + transaction.get_amount();
            let mod_recipient_balance: f32 = self.get_balance(&recipient).unwrap() - transaction.get_amount();

            // update both modified balances
            self.db.put(&bincode::serialize(&sender).unwrap(), &mod_sender_balance.to_le_bytes())?;
            self.db.put(&bincode::serialize(&recipient).unwrap(), &mod_recipient_balance.to_le_bytes())?;
        }

        Ok(())
    }

    /// Reads and returns the balance of a given adress.
    /// 
    /// # Arguments
    /// * `public_key` - A &Point which specifies a reference to the public key to lookup
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling get on it.
    /// 
    /// # Returns
    /// An Option<f32> which is the balance of the address if it exists in the db, or None if it does not.
    /// 
    pub fn get_balance(&mut self, public_key: &Point) -> Option<f32> {

        // serialize the public_key to get the database key. Unwrap because Point never fails to serialize
        match self.db.get(&bincode::serialize(public_key).unwrap()) {
            Some(bytes) => {

                // wrap bytes buffer with a cursor for easy little-endian conversion to f32
                let mut reader: Cursor<Vec<u8>> = Cursor::new(bytes);

                Some(reader.read_f32::<LittleEndian>().unwrap()) // Panic if data is corrupted
            },
            None => None // address was not found
        }
    }

    /// Reads and returns the block with a specific height if it exists
    /// 
    /// # Arguments
    /// * `height` - A u64 that specifies the index of the block in the DB
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling get on it.
    /// 
    /// # Returns
    /// An Option<Block> which is the block at the specified height if it exists in the db, or None if it does not.
    /// 
    pub fn get_block(&mut self, height: u64) -> Result<Block, Status> {
        // convert height to little-endian for standard use throughout project
        match self.db.get(&height.to_le_bytes()) {
            Some(bytes) => {
                let block: Block = bincode::deserialize(&bytes).map_err(|e| 
                    Status::new(rusty_leveldb::StatusCode::Corruption, &format!("{e}"))
                )?; 

                Ok(block)
            },
            None => { 
                Err(Status::new(rusty_leveldb::StatusCode::NotFound, &format!("Block not found"))) 
            }
        }
    }

    /// Obtain latest block from the on-machine node database
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling get on it.
    /// 
    /// # Returns
    /// An Option<Block> which is the latest block in the db, or None if it does not exist.
    /// 
    pub fn get_latest_block(&mut self) -> Result<Block, Status> {

        match self.db.get(LATEST_BLOCK_KEY) {
            Some(bytes) => {

                // attempt to desiralize the block
                let block: Block = bincode::deserialize(&bytes).map_err(|e| 
                    Status::new(rusty_leveldb::StatusCode::Corruption, &format!("{e}"))
                )?;

                Ok(block)
            },
            None => { 
                Err(Status::new(rusty_leveldb::StatusCode::NotFound, &format!("Block not found"))) 
            }
        }
    }

    /// Deletes the block at index height in db. 
    /// Used mainly for pruning.
    /// 
    /// # Arguments
    /// * `height` - A u64 that specifies the index of the block in the DB
    /// 
    /// # Modifications
    /// This method modifies the db object by deleting an entry from it.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the block was successfully deleted, or an error if it was not.
    ///
    pub fn delete_block(&mut self, height: u64) -> Result<(), Status> {
        // convert height to little-endian for standard use throughout project
        self.db.delete(&height.to_le_bytes())?;

        // makes new genesis block if no latest block
        let latest_block: Block = self.get_latest_block().unwrap_or(Block::new_genesis());

        // reverse transactions if delete was successful and if deleting latest block
        if height == latest_block.get_height() {
            let new_latest_block: Block = self.get_block(height - 1).unwrap(); // panic if no other block
            self.update_latest_block(&new_latest_block)?;

            self.reverse_chainstate(latest_block.get_transactions())?; // No transactions in genesis, so no harm done
        }
        
        Ok(())
    }


    
}
