extern crate sha2;
extern crate ecdsa;

use sha2::sha256::hash;
use ecdsa::secp256k1::modular_multiplicative_inverse;
use num_bigint::{BigInt, ToBigInt};


fn main() {
    print!("{}\n", hash(String::from("hello world")));

    let five: BigInt = 389.to_bigint().unwrap();
    let seven: BigInt = 140.to_bigint().unwrap();
    print!("{}\n", modular_multiplicative_inverse(five, seven, None, None, None, None).to_string());
}
