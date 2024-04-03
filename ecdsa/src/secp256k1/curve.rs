use num_bigint::BigInt;
use super::Point;

#[derive(Debug)]
pub struct Curve {
    pub p: BigInt,
    pub n: BigInt,
    pub g: Point
}

impl Curve {
    pub fn new() -> Curve {
        let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };

        Curve {
            p: bigint(super::P),
            n: bigint(super::N),
            g: Point {
                x: bigint(super::X),
                y: bigint(super::Y),
                fp: bigint(super::FP),
            }}
    }
}