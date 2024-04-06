use std::collections::VecDeque;
use sha256::hash;

pub struct MerkelTree {
    pub leaves: Vec<String>,
    pub root: String
}

impl MerkelTree {
    pub fn new(leaves: Vec<String>) -> MerkelTree {
        MerkelTree {
            leaves: leaves.clone(),
            root: MerkelTree::get_root(leaves)
        }
    }
 

    fn get_root(mut leaves: Vec<String>) -> String {
        // duplicate last element if odd number of leaves
        if leaves.len() % 2 == 1 {
            leaves.push(leaves[leaves.len() - 1].clone()); 
        }

        let mut queue: VecDeque<String> = VecDeque::new();
        queue.extend(leaves);

        while queue.len() > 1 {
            let f: String = queue.pop_front().unwrap_or_default();
            let s: String = queue.pop_front().unwrap_or_default();

            queue.push_back(hash(f + &s));
        }

        queue[0].clone()
    }
}