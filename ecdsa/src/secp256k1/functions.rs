use num_bigint::BigInt;
use num_traits::zero;
use super::{Curve, Point};
use crate::math::{modulo, entropy};

/*
    returns the x coordinate as a compressed point (essentially the public key)
*/
pub fn compress_point(point: Point) -> String {
    let mut prefix: String;

    if &point.y % 2 != zero() {
        prefix = String::from("03");
    } else {
        prefix = String::from("02");
    }

    let hex_point: String = format!("{:x}", point.x);

    if hex_point.len() < 64 {
        prefix.push_str("0");
    }
    prefix.push_str(&hex_point);

    prefix
}

/*

*/
pub fn sign(message: &str, k: Option<BigInt>) {
    let secp256k1: Curve = super::Curve::new();

    let k: BigInt = k.unwrap_or(modulo(&entropy(), &secp256k1.p));

    let p: Point = secp256k1.g.multiply(k);
}