use num_bigint::BigInt;
use crate::math::bigint;

use super::Point;

#[derive(Debug, Clone)]
pub struct Curve {
    pub p: BigInt,
    pub n: BigInt,
    pub g: Point,
}

impl Curve {
    
    /*
        returns a secp256k1 curve with all the right default params
    */
    pub fn new() -> Self {
        Curve {
            p: bigint(super::P),
            n: bigint(super::N),
            g: Point {
                x: bigint(super::X),
                y: bigint(super::Y)
            }
        }
    }
}