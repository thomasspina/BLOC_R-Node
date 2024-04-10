const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
const X: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const Y: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
const FP: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"; // prime field for points

pub const W: u32 = 4;

// precomputes the points for the curve only once and stores it in a static ref
mod precomputed_points {
    use super::{point::precompute_points, Curve, Point, W};

    pub static PRECOMPUTED_POINTS: once_cell::sync::Lazy<Vec<Point>> =
        once_cell::sync::Lazy::new(|| {
            precompute_points(Curve::new().g, W)
        });
}

// returns a reference to the secp256k1 precomputed points
pub fn get_curve_computed_points() -> &'static Vec<Point> {
    &precomputed_points::PRECOMPUTED_POINTS
}

mod curve;
mod point;
mod signature;

pub use curve::Curve;
pub use point::Point;
pub use signature::{Signature, sign, verify_signature};
