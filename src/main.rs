use std::env;
use ecdsa::secp256k1::{self, Curve, Point};
use num_bigint::BigInt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let private_key = args[1].clone();
    // let message = args[2].clone();

    let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };

    let secp256k1: Curve = Curve::new();

    let public_key: Point = secp256k1.g.multiply(bigint(&private_key.clone()));

    println!("\nPublic key:\n\n{}\n", secp256k1::compress_point(public_key));
}
