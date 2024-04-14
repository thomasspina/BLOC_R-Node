use core::fmt;
use ecdsa::secp256k1::{sign, verify_signature, Point, Signature};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use super::REWARD;

#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    sender: Point,
    recipient: Point,
    amount: f32,
    signature: Signature
}

/*
    adds to_string for Signature struct
*/
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
    /*
        generates a reward transaction for the miner that doesn't need to be signed
    */
    pub fn reward_transaction(recipient: &Point) -> Self {
        Transaction {
            sender: Point::identity(),
            recipient: recipient.clone(),
            amount: REWARD,
            signature: Signature::get_empty()
        }
    }

    /*
        returns a new transaction that has already been signed using the private key
    */
    pub fn new(sender: &Point, recipient: &Point, amount: f32, private_key: &BigInt) -> Self {
        let message: String = sender.to_string() + &recipient.to_string() + &amount.to_string();
        let signature: Signature = sign(&message, private_key.clone(), Some(BigInt::from(90127834)));

        Transaction {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount,
            signature
        }
    }

    /*
        returns sender's public key 
    */
    pub fn get_sender(&self) -> Point { self.sender.clone() }

    /*
        returns recipient's public key 
    */
    pub fn get_recipient(&self) -> Point { self.recipient.clone() }

    /*
        returns transaction amount 
    */
    pub fn get_amount(&self) -> f32 { self.amount.clone() }

    /*
        returns transaction signature 
    */
    pub fn get_signature(&self) -> Signature { self.signature.clone() }

    /*
        verifies the current signature
    */
    pub fn verify(&self) -> bool {
        verify_signature(&self.signature, &self.get_message(), self.sender.clone())
    }

    /*
        gets the transaction's message that was used in the signature
    */
    fn get_message(&self) -> String {
        self.sender.to_string() + &self.recipient.to_string() + &self.amount.to_string()
    }

    /*
        gets the transactions hash, used for merkel root
    */
    pub fn get_hash(&self) -> String {
        sha256::hash(format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature))
    }
}

