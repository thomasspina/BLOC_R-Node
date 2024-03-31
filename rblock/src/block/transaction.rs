use sha2::sha256::hash as hash_256;

struct Transaction {
    hash: String,
    sender: String,
    recipient: String,
    amount: f32,
    signature: String
}

impl Transaction {
    fn hash(&mut self) -> String {
        self.hash = hash_256(format!("{}{}{}{}", sender, recipient, amount, signature));
    }
}

