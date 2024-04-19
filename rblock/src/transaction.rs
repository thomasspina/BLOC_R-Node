use core::fmt;
use ecdsa::secp256k1::{sign, verify_signature, Point, Signature};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use super::REWARD;

/// A transaction in the blockchain
#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    /// The public key of the sender
    sender: Point,

    /// The public key of the recipient
    recipient: Point,

    /// The amount of the transaction
    amount: f32,

    /// The digital signature of the transaction, signed by the sender
    signature: Signature
}

/// implement display for transaction struct for easy printing
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tsender: {}\n\trecipient: {}\n\tamount: {}\n\tsignature: {}", 
            self.sender, 
            self.recipient,
            self.amount,
            self.signature)
    }
}

impl Transaction { 
    /// generates a reward transaction for the miner that doesn't need to be signed
    /// 
    /// # Arguments
    /// * `recipient` - the public key of the miner
    /// 
    /// # Returns
    /// * a new transaction with the reward amount
    /// 
    pub fn reward_transaction(recipient: &Point) -> Self {
        Transaction {
            sender: Point::identity(),
            recipient: recipient.clone(),
            amount: REWARD,
            signature: Signature::get_empty()
        }
    }

    /// returns a new transaction that has already been signed using the private key
    /// 
    /// # Arguments
    /// * `sender` - the public key of the sender
    /// * `recipient` - the public key of the recipient
    /// * `amount` - the amount of the transaction
    /// * `private_key` - the private key of the sender, used to sign the transaction
    /// 
    /// # Returns
    /// * a new transaction with the sender, recipient, amount, and signature
    /// 
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

    /// returns the sender's public key
    pub fn get_sender(&self) -> Point { self.sender.clone() }

    /// returns the recipient's public key
    pub fn get_recipient(&self) -> Point { self.recipient.clone() }

    /// returns the amount of the transaction
    pub fn get_amount(&self) -> f32 { self.amount.clone() }

    /// returns the signature of the transaction
    pub fn get_signature(&self) -> Signature { self.signature.clone() }

    /// verifies the signature of the transaction
    /// 
    /// # Returns
    /// * true if the signature is valid, false otherwise
    /// 
    pub fn verify(&self) -> bool {
        verify_signature(&self.signature, &self.get_message(), self.sender.clone())
    }

    /// returns the message that was signed
    fn get_message(&self) -> String {
        self.sender.to_string() + &self.recipient.to_string() + &self.amount.to_string()
    }

    /// returns the hash for the transaction, used in the block's merkel root exclusively
    pub fn get_hash(&self) -> String {
        sha256::hash(format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature))
    }
}

