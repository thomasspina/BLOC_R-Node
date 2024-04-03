use std::env;
use ecdsa::secp256k1::{verify_signature, Curve, Point, sign};
use num_bigint::BigInt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let private_key = args[1].clone();
    let message = args[2].clone();

    let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };

    let secp256k1: Curve = Curve::new();

    let public_key: Point = secp256k1.g.multiply(bigint(&private_key.clone()));

    println!("{}", verify_signature(&sign(&message, bigint(&private_key), None), &message, public_key));
}
