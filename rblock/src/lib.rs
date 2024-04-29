const BLOCK_SPEED: u64 = 1200; // 20 min between blocks
const TRANSACTION_LIMIT_PER_BLOCK: usize = 5000;
const REWARD: f32 = 1.5;

mod block;
pub use block::Block;

mod functions;

mod transaction;
pub use transaction::Transaction;
pub use functions::get_merkel_root;
