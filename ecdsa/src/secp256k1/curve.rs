use num_bigint::BigInt;
use crate::math::bigint;
use super::Point;

/// Curve struct that holds the parameters of the secp256k1 curve
/// The constants for the curve are defined in the super module
pub struct Secp256k1 {
    /// The prime number that defines the field the curve is over
    pub p: BigInt,

    /// The order of the curve
    pub n: BigInt,

    /// The generator point
    pub g: Point,
}

impl Secp256k1 {
    
    /// Returns a new instance of the Secp256k1 struct
    pub fn new() -> Self {
        Secp256k1 {
            p: bigint(super::P),
            n: bigint(super::N),
            g: Point {
                x: bigint(super::X),
                y: bigint(super::Y)
            }
        }
    }
}