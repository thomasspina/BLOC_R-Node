use std::{collections::VecDeque, time::{Duration, SystemTime, UNIX_EPOCH}};
use sha256::hash;
use super::Transaction;

/// returns the current unix time
/// https://en.wikipedia.org/wiki/Unix_time
pub fn get_unix_time() -> u64 {
    let current_time: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = current_time.duration_since(UNIX_EPOCH).unwrap();

    duration_since_epoch.as_secs()
}

/// returns the merkel root of all the transactions
/// https://en.wikipedia.org/wiki/Merkle_tree
/// 
/// # Arguments
/// * `transactions` - A vector of transactions
/// 
/// # Returns
/// * A string representing the merkel root
/// 
pub fn get_merkel_root(transactions: &Vec<Transaction>) -> String {
    if transactions.len() == 0 {
        return "".to_owned();
    }
    
    let mut nodes: VecDeque<String> = VecDeque::new();
    for transaction in transactions {
        nodes.push_back(transaction.get_hash());
    }

    // duplicate last element if odd number of leaves
    if nodes.len() % 2 == 1 {
        nodes.push_back(nodes[nodes.len() - 1].clone()); 
    }

    while nodes.len() > 1 {
        let f: String = nodes.pop_front().unwrap_or_default();
        let s: String = nodes.pop_front().unwrap_or_default();

        nodes.push_back(hash(f + &s));
    }

    nodes[0].clone()
}