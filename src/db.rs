use std::{collections::HashMap, io::{self, ErrorKind}, path::PathBuf};
use dirs::home_dir;
use ecdsa::secp256k1::Point;
use rblock::{Block, Transaction};
use rusty_leveldb::{DBIterator, LdbIterator, Options, Status, DB};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub const DB_FILENAME: &'static str = ".r_blocks";
const LATEST_BLOCK_KEY: &'static [u8; 6] = b"latest";
const PUBLIC_KEY_PREFIX: &'static [u8; 7] = b"userPK_";


/// A struct that represents a database of blocks.
/// 
/// # Fields
/// * `db` - A DB object that represents the database of blocks
/// 
pub struct BlocksDB {
    db: DB
}

impl BlocksDB {
    /// Starts the database and returns a BlocksDB object with the database
    /// 
    /// # Modifications
    /// This method creates a new database file in the home directory of the user.
    /// 
    pub fn start_db() -> Result<Self, Status> {
        let mut options: Options = Options::default();
        options.create_if_missing = false; // create DB if missing
        
        // get home directory
        let path: PathBuf = home_dir().ok_or_else(|| {
            io::Error::new(ErrorKind::NotFound, "Home directory could not be found")
        })?;

        let db: DB = DB::open(path.join(DB_FILENAME), options)?;
        Ok(BlocksDB { db })
    }


    // TODO: don't forget to remove this function
    pub fn init_db(&mut self, point1: &Point, point2: &Point) {
        let genesis: Block = Block::new_genesis();

        self.put_block(&genesis).unwrap();
        self.update_latest_block(&genesis).unwrap();

        self.update_balance(point1, 10.).unwrap();
        self.update_balance(point2, 10.).unwrap();
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
        self.db.flush()?;

        Ok(())
    }

    /// Puts a block into the db if it doesn't exist already.
    /// 
    /// # Arguments
    /// * `block` - A &Block which specifies a reference to the block to put into the db
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling put on it.
    /// 
    /// # Returns
    /// An Result<bool, Status> which is Ok(true) if the block was successfully put, or Ok(false) if it already existed.
    /// 
    fn put_block(&mut self, block: &Block) -> Result<(), Status> {
        match self.get_block(block.get_height()) {
            Ok(_) => {
                Err(Status::new(rusty_leveldb::StatusCode::AlreadyExists, &format!("Block already exists in db")))
            },
            Err(e) => {
                if e.code == rusty_leveldb::StatusCode::NotFound {
                    // serialize block
                    let binary: Vec<u8> = bincode::serialize(block).unwrap(); // blocks are always serializable
                    self.db.put(&block.get_height().to_le_bytes(), &binary)?;
                    self.db.flush()?;
                    
                    // successful put
                    return Ok(());
                }

                Err(e)
            }
        }
    }


    /// Adds a block into the db if it doesn't already exist.
    /// Method should only be used to add a new highest block. It doesn't allow blocks other than the next one over to be added
    /// 
    /// # Arguments
    /// * `block` - A &Block which specifies a reference to the block to put into the db
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling put on it.
    /// 
    pub fn add_block(&mut self, block: &Block) -> Result<(), Status> {
        let latest_block: Block = self.get_latest_block()?;
        let latest_block_height: u64 = latest_block.get_height();
        let added_block_height: u64 = block.get_height();

        // check if genesis
        if added_block_height == 0 {
            return Err(Status::new(rusty_leveldb::StatusCode::NotSupported, &format!("Cannot add another genesis block")));
        }

        // if the latest block is smaller than added block
        if latest_block_height == added_block_height - 1 {

            // update db with new latest block info
            self.update_latest_block(block)?;
            self.update_chainstate(block.get_transactions())?;

        // if latest block is much smaller than added block
        } else if latest_block_height < added_block_height - 1 {
            return Err(Status::new(rusty_leveldb::StatusCode::NotSupported, &format!("Block height is greater next latest block.")));
        } else if latest_block_height >= added_block_height {
            return Err(Status::new(rusty_leveldb::StatusCode::NotSupported, &format!("Block height is much smaller than latest block's")));
        }
        
        // put block after all checks otherwise there could be some issues
        self.put_block(block)?;
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
    pub fn get_balance(&mut self, public_key: &Point) -> Result<f32, Status> {

        // serialize the public_key to get the database key. Unwrap because Point never fails to serialize
        match self.db.get(&BlocksDB::get_db_user_key(public_key)) {
            Some(bytes) => {

                // wrap bytes buffer with a cursor for easy little-endian conversion to f32
                let mut reader: Cursor<Vec<u8>> = Cursor::new(bytes);

                Ok(reader.read_f32::<LittleEndian>()?) // Error if data is corrupted
            },
            None => {
                // address was not found
                Err(Status::new(rusty_leveldb::StatusCode::NotFound, &format!("Public key was not found.")))
            } 
        }
    }

    /// Method used to update balance for an existing user or create a new user with a specified balance.
    /// 
    /// # Arguments
    /// * `public_key` - A &Point which specifies a reference to the public key to update the balance of
    /// * `value` - A f32 which specifies the value to update the balance to
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling put on it.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the balance was successfully updated, or an error if it was not.
    /// 
    fn update_balance(&mut self, public_key: &Point, value: f32) -> Result<(), Status> {
        // account balances are stored in little-endian
        self.db.put(&BlocksDB::get_db_user_key(public_key), &value.to_le_bytes())?;
        self.db.flush()?;
        Ok(())
    }


    /// Method to add prefix to the user public keys to get the key in the db
    /// 
    /// # Arguments
    /// * `public_key` - A &Point which specifies a reference to the public key to get the db key for
    /// 
    /// # Returns
    /// A Vec<u8> which is the key in the db for the public key
    /// 
    fn get_db_user_key(public_key: &Point) -> Vec<u8> {
        // add prefix
        let mut key: Vec<u8> = Vec::new();
        key.extend_from_slice(PUBLIC_KEY_PREFIX);
        key.extend_from_slice(&bincode::serialize(public_key).unwrap());

        key
    }

    
    /// verifies that the transactions are valid and can be added to the chainstate.
    /// makes a hashmap of all the new balances so that the new balances can be updated easily
    /// 
    /// # Arguments
    /// * `transactions` - A Vec<Transaction> which specifies the transactions to verify
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling get on it.
    /// 
    /// # Returns
    /// An Result<HashMap<&Point, f32>, Status> which is returns a map of the new balances of the verified transactions
    /// 
    fn verify_transactions(&mut self, transactions: &Vec<Transaction>) -> Result<HashMap<Point, f32>, Status> {
        // hashmap to remember good balances
        let mut balances: HashMap<Point, f32> = HashMap::new();

        for transaction in transactions {
            let sender: Point = transaction.get_sender();
            let recipient: Point = transaction.get_recipient();

            // Point::identity is miner reward
            if sender != Point::identity() {

                // get original balances
                // check hashmap first for balances
                let sender_balance: f32 = *balances.get(&sender).unwrap_or(
                    &self.get_balance(&sender).unwrap_or(0.0)
                );

                // calculate new balances
                let new_sender_balance: f32 = sender_balance - transaction.get_amount();

                balances.insert(sender, new_sender_balance);
            }

            // do same for recipient
            let recipient_balance: f32 = *balances.get(&recipient).unwrap_or(
                &self.get_balance(&recipient).unwrap_or(0.0)
            );
            
            let new_recipient_balance: f32 = recipient_balance + transaction.get_amount();
            
            balances.insert(recipient, new_recipient_balance);
        }

        for (addr, balance) in balances.iter() {
            if *balance < 0.0 {
                return Err(Status::new(rusty_leveldb::StatusCode::InvalidData, &format!("public key: {} has negative balance after all transactions.", addr)));
            }
        }

        Ok(balances)
    }

    
    /// Updates the chainstate with the transactions of a given block.
    /// Multiple checks should be made before using this method. Method is private so as to not invalidate the data in the db
    /// If error on update balance. Chainstate should be rebuilt from beginning
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
        let verified_balances: HashMap<Point, f32> = self.verify_transactions(&transactions)?; 

        // update all balances
        for (addr, balance) in verified_balances.iter() {
            self.update_balance(addr, *balance)?;
        }

        Ok(())
    }

    /// Method used to wipe the chainstate of every user and transaction.
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling delete on it.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the chainstate was successfully cleared, or an error if it was not.
    /// 
    fn clear_chainstate(&mut self) -> Result<(), Status> {
        // make new db iterator
        let mut iter: DBIterator = self.db.new_iter()?;

        // init values for key and val
        let mut key: Vec<u8> = vec![];
        let mut val: Vec<u8> = vec![];

        while iter.advance() {
            // get current key and val
            iter.current(&mut key, &mut val);
            
            // verify prefix
            if key.len() >= 7 && key[0..7] == *PUBLIC_KEY_PREFIX {
                // wipe key
                self.db.delete(&key)?;
            }
        }

        Ok(())
    }

    /// Method used to rebuild chainstate from all the blocks in the db.
    /// Used in case when chainstate gets updated, there are some internal errors and the data gets corrupted
    /// 
    /// # Modifications
    /// This method changes the internal state of the DB object by calling get and put on it.
    /// 
    /// # Returns
    /// An Result<(), Status> which is Ok(()) if the chainstate was successfully rebuilt, or an error if it was not.
    /// 
    pub fn rebuild_chainstate(&mut self) -> Result<(), Status> {

        // clear chainstate
        self.clear_chainstate()?;

        let mut curr_block: Block; 
        let mut curr_height: u64 = 0; // start at genesis block

        let latest_block: Block = self.get_latest_block()?;
        let latest_block_height: u64 = latest_block.get_height();
    

        while curr_height <= latest_block_height {
            // get block
            curr_block = self.get_block(curr_height)?;

            // update chainstate
            let transactions: Vec<Transaction> = curr_block.get_transactions();
            self.update_chainstate(transactions)?;

        
            curr_height += 1;
        }

        Ok(())
    }
}
