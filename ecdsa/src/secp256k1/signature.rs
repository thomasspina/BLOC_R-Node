//! for more info on the maths here: https://cryptobook.nakov.com/digital-signatures/ecdsa-sign-verify-messages

use core::fmt;
use num_bigint::BigInt;
use num_traits::zero;
use sha256::hash;
use super::{Secp256k1, Point, W};
use crate::{math::{self, bigint, entropy, modular_multiplicative_inverse, modulo}, 
            secp256k1::get_curve_precomputed_points};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer};

/// Signature struct that holds the r and sigma values of a digital signature
#[derive(Clone)]
pub struct Signature {
    r: BigInt,
    s: BigInt
}

impl Signature {

    /// returns an empty signature (empty meaning with r and s as 0)
    /// 
    /// # Returns
    /// an empty signature struct 
    /// 
    pub fn get_empty() -> Self {
        Signature { r: zero(), s: zero() }
    }
}

/// implement for serialization for Signature
/// manual implementation needed because BigInt is not directly serializable
/// implementation is done by serializing the bigint as a hex string
impl Serialize for Signature {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer 
    {
        let mut state = serializer.serialize_struct("Signature", 2)?;

        // encode bigint as hex
        state.serialize_field("r", &format!("{:x}", &self.r))?; 
        state.serialize_field("s", &format!("{:x}", &self.s))?;
        state.end()
    }
}

/// implement for deserialization for Signature
/// manual implementation needed because BigInt is not directly deserializable
/// implementation is done by deserializing the hex string as a bigint
impl<'de> Deserialize<'de> for Signature {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        // need to use intermediary struct with strings because 
        // bigint is not directly serializable and desrializable
        #[derive(serde::Deserialize)]
        struct SignatureFields {
            r: String,
            s: String
        }

        let fields: SignatureFields = SignatureFields::deserialize(deserializer)?;

        Ok(Signature {
            r: math::bigint(&fields.r),
            s: math::bigint(&fields.s)
        })
    }
}


/// implement for display for Signature
/// this is done to make it easier to print the signature
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}_s{}", self.r, self.s)
    }
}

/// returns signature using "sigma = ( H(m) + n * rx ) / k"
/// 
/// # Arguments
/// * `message` - A string slice that holds the message to be signed
/// * `d` - A BigInt that is the private key
/// * `k` - An optional BigInt that is the nonce
/// 
/// # Returns
/// A Signature struct that holds the r and sigma values of the signature
/// 
pub fn sign(message: &str, d: BigInt, k: Option<BigInt>) -> Signature {
    let secp256k1: Secp256k1 = Secp256k1::new(); // gets parameters for secp256k1 curve

    let k: BigInt = k.unwrap_or(modulo(&entropy(), &secp256k1.p));

    let p: Point = secp256k1.g.multiply(k.clone(), W, get_curve_precomputed_points());

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


/// verifies a signature using "r = (u1 * G + u2 * Q).x"
/// 
/// # Arguments
/// * `signature` - A reference to a Signature struct that holds the r and sigma values of the signature
/// * `message` - A string slice that holds the message to be signed
/// * `public_key` - A Point struct that is the public key
/// 
/// # Returns
/// A boolean that is true if the signature is valid and false otherwise
/// 
pub fn verify_signature(signature: &Signature, message: &str, public_key: Point) -> bool {
    let secp256k1: Secp256k1 = Secp256k1::new(); // gets parameters for secp256k1 curve

    let z: BigInt = bigint(&hash(message.to_owned() + &secp256k1.p.to_string()));

    let w: BigInt = modulo(&modular_multiplicative_inverse(&secp256k1.n, signature.s.clone(), None, None), 
                            &secp256k1.n);

    let u1: BigInt = modulo(&(z * &w), &secp256k1.n);
    let u2: BigInt = modulo(&(&signature.r * &w), &secp256k1.n); 

    let p1: Point = secp256k1.g.multiply(u1, W, get_curve_precomputed_points());
    let public_key_precomp: Vec<Point> = super::point::precompute_points(public_key.clone(), W);

    let p2: Point = public_key.multiply(u2.clone(), W, &public_key_precomp);

    let res: Point = p1.add(&p2);

    res.x.eq(&signature.r)
}
