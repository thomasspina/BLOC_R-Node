// for more info on the maths here: https://cryptobook.nakov.com/digital-signatures/ecdsa-sign-verify-messages

use core::fmt;
use num_bigint::BigInt;
use num_traits::zero;
use sha256::hash;
use super::{Curve, Point};
use crate::math::{bigint, entropy, modular_multiplicative_inverse, modulo};

#[derive(Clone)]
pub struct Signature {
    r: BigInt,
    s: BigInt
}

// adds to_string for Signature struct
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{} s{}", self.r, self.s)
    }
}

/*
    returns signature using 
        sigma = ( H(m) + n * rx ) / k
    note that d is the private key here and k is a nonce
*/
pub fn sign(message: &str, d: BigInt, k: Option<BigInt>) -> Signature {
    let secp256k1: Curve = super::Curve::new(); // gets parameters for secp256k1 curve

    let k: BigInt = k.unwrap_or(modulo(&entropy(), &secp256k1.p));

    let p: Point = secp256k1.g.multiply(k.clone());

    let r: BigInt = modulo(&p.x, &secp256k1.p);
    if r == zero() {
        return sign(message, d, Some(k));
    }

    let m: String = hash(message.to_owned() + &secp256k1.p.to_string());

    let sigma: BigInt = modulo(&((&d * &r + bigint(&m)) * 
                            modular_multiplicative_inverse(&secp256k1.n, k.clone(), None, None)), 
                            &secp256k1.n);

    if sigma == zero() {
        return sign(message, d, Some(k));
    }

    Signature { r, s: sigma }
}


/*
    checks whether the signature is from the public key or not
    the math is hard to grasp, but thankfully its not too many operations.
*/
pub fn verify_signature(signature: &Signature, message: &str, public_key: Point) -> bool {
    let secp256k1: Curve = super::Curve::new(); // gets parameters for secp256k1 curve

    let z: BigInt = bigint(&hash(message.to_owned() + &secp256k1.p.to_string()));

    let w: BigInt = modulo(&modular_multiplicative_inverse(&secp256k1.n, signature.s.clone(), None, None), 
                            &secp256k1.n);

    let u1: BigInt = modulo(&(z * &w), &secp256k1.n);
    let u2: BigInt = modulo(&(&signature.r * &w), &secp256k1.n); 

    let p1: Point = secp256k1.g.multiply(u1);
    let p2: Point = public_key.multiply(u2);

    let res: Point = p1.add(&p2);

    res.x.eq(&signature.r)
}
