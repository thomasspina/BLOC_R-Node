use core::fmt;
use ecdsa::secp256k1::{sign, verify_signature, Point, Signature};
use num_bigint::BigInt;
use serde::ser::{Serialize, SerializeStruct};
use super::REWARD;

/*
    TODO: transactions need to be verified to make sure that the 
    sender is not sending a bunch of money that they don't have
    --> update on that: apparently that is done through a chainstate db
        which stores all the information about every adress and the amount of coins they have
*/

#[derive(Clone)]
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

/*
    implement for json serialization
*/
impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_struct("Transaction", 4)?;
        state.serialize_field("sender", &self.sender)?;
        state.serialize_field("recipient", &self.recipient)?;
        state.serialize_field("amount", &self.amount)?;
        state.serialize_field("signature", &self.signature)?;
        state.end()
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

