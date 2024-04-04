use ecdsa::secp256k1;
use num_bigint::BigInt;

pub struct Transaction {
    hash: String,
    sender: String,
    recipient: String,
    amount: f32,
    signature: secp256k1::Signature
}

impl Transaction { 
    /*
        signs the transaction and updates it using the secret_key provided
    */
    fn sign(&mut self, secret_key: &BigInt) {
        let message: &str = &(self.sender.clone() + &self.recipient + &self.amount.to_string() + &self.signature.to_string());
        self.signature = secp256k1::sign(message, secret_key.clone(), None);
    }

    /*
    
    */
    fn verify(&self, public_key: String) -> bool {
        
        return false;
    }

    /*
        returns the hash for the transaction
    */
    fn get_hash(&self) -> String {
        sha256::hash(format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.signature))
    }
}

