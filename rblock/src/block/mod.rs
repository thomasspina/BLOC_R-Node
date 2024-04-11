const BLOCK_SPEED: u64 = 20; // 20 seconds between blocks
const MEAN_BLOCK_COUNT: u32 = 10; // 10 blocks mean
const REWARD: f32 = 1.5;

mod block;
pub use block::Block;

mod blockchain;
pub use blockchain::Blockchain;

mod functions;

mod transaction;
pub use transaction::Transaction;