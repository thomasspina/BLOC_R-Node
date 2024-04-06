use ecdsa::secp256k1::{sign, verify_signature, Point, Signature};
use num_bigint::BigInt;
use num_traits::zero;

#[derive(Clone)]
pub struct Transaction {
    pub sender: Point,
    pub recipient: Point,
    pub amount: f32,
    pub signature: Signature
}

impl Transaction { 
    pub fn new(sender: Point, recipient: Point, amount: f32) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount,
            signature: Signature { r: zero(), s: zero() }
        }
    }

    pub fn sign(&mut self, secret_key: &BigInt) {
        self.signature = sign(&self.get_message(), secret_key.clone(), None);
    }

    pub fn verify(&self, public_key: Point) -> bool {
        verify_signature(&self.signature, &self.get_message(), public_key)
    }

    pub fn get_message(&self) -> String {
        self.sender.to_string() + &self.recipient.to_string() + &self.amount.to_string()
    }

    pub fn get_hash(&self) -> String {
        sha256::hash(format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature))
    }
}

