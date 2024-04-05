use ecdsa::secp256k1::{self, Point};
use num_bigint::BigInt;

pub struct Transaction {
    pub hash: String,
    pub sender: secp256k1::Point,
    pub recipient: secp256k1::Point,
    pub amount: f32,
    pub signature: secp256k1::Signature
}

impl Transaction { 
    pub fn sign(&mut self, secret_key: &BigInt) {
        self.signature = secp256k1::sign(&self.get_message(), secret_key.clone(), None);
    }

    pub fn verify(&self, public_key: Point) -> bool {
        secp256k1::verify_signature(&self.signature, &self.get_message(), public_key)
    }

    pub fn get_message(&self) -> String {
        self.sender.to_string() + &self.recipient.to_string() + &self.amount.to_string()
    }

    pub fn get_hash(&self) -> String {
        // TODO : causes subtraction overflow?
        //sha256::hash(format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature))
        format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature)
    }
}

