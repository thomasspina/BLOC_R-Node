use ecdsa::secp256k1::{sign, verify_signature, Point, Signature};
use num_bigint::BigInt;

#[derive(Clone)]
pub struct Transaction {
    sender: Point,
    recipient: Point,
    amount: f32,
    signature: Signature
}

impl Transaction { 
    pub fn new(sender: &Point, recipient: &Point, amount: f32, private_key: &BigInt) -> Self {
        let message: String = sender.to_string() + &recipient.to_string() + &amount.to_string();
        let signature: Signature = sign(&message, private_key.clone(), None);

        Transaction {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount,
            signature
        }
    }

    pub fn get_sender(&self) -> &Point { &self.sender }
    pub fn get_recipient(&self) -> &Point { &self.recipient }
    pub fn get_amount(&self) -> &f32 { &self.amount }

    pub fn verify(&self) -> bool {
        verify_signature(&self.signature, &self.get_message(), self.sender.clone())
    }

    fn get_message(&self) -> String {
        self.sender.to_string() + &self.recipient.to_string() + &self.amount.to_string()
    }

    pub fn get_hash(&self) -> String {
        sha256::hash(format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature))
    }
}

