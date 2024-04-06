use std::env;
use ecdsa::secp256k1::{Curve, Point, Signature};
use num_traits::zero;
use rblock::block::Transaction;
use num_bigint::BigInt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let private_key1 = args[1].clone();
    let private_key2 = args[2].clone();

    let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };

    let secp256k1: Curve = Curve::new();
    let public_key1: Point = secp256k1.g.multiply(bigint(&private_key1.clone()));

    let secp256k1_2: Curve = Curve::new();
    let public_key2: Point = secp256k1_2.g.multiply(bigint(&private_key2.clone()));


    let mut t = Transaction {
        sender: public_key1.clone(),
        recipient: public_key2,
        amount: 10.00,
        signature: Signature{r: zero(), s: zero()},
    };

    
    t.sign(&bigint(&private_key1.clone()));
    print!("{}",t.verify(public_key1));

}