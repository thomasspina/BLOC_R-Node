use std::env;
use ecdsa::secp256k1::curve::{Curve, Point};
use num_bigint::BigInt;


fn main() {
    let args: Vec<String> = env::args().collect();

    let private_key = args[1].clone();
    // let message = args[2].clone();

    let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };

    let secp256k1 = Curve {
        p: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"),
        n: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",),
        g: Point {
            x: bigint("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"),
            y: bigint("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"),
            fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"),
        },
    };

    let _public_key = secp256k1.g.multiply(bigint(&private_key.clone()));

    //println!("\nPublic key:\n\n{}\n", compress_point(&public_key));
}
