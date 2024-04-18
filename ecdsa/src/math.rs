use num_bigint::{BigInt, Sign};
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use num_traits::{zero, one};
use num_traits::ToPrimitive;

/// Helper functions for BigInt operations
/// 
/// # Arguments
/// * `num` - A string slice that holds the number to be converted to BigInt
/// 
/// # Returns
/// A BigInt representation of the number
/// 
pub fn bigint(num: &str) -> BigInt {
    BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
}

/// Helper function to get a modulo in the euclidean sense
/// 
/// # Arguments
/// * `x` - A reference to a BigInt the number to be modded
/// * `m` - A reference to a BigInt being the modulo
/// 
/// # Returns
/// A BigInt representation of the modulo
/// 
pub fn modulo(x: &BigInt, m: &BigInt) -> BigInt {
    ((x % m) + m) % m
}

/// Helper function to calculate the w-ary non-adjacent form of a number
/// 
/// # Arguments
/// * `w` - A u32 that is the window size
/// * `n` - A BigInt that is the number to be converted to wnaf
/// 
/// # Returns
/// A Vec<i8> representation of the wnaf
/// 
pub fn calculate_wnaf(w: u32, mut n: BigInt) -> Vec<i8> {
    let mut wnaf: Vec<i8> = Vec::new();

    let modulus: BigInt = BigInt::from(1 << w);
    let mut i: usize = 0;

    while n >= one() {
        // if n is odd
        if &n & &one() == one() {
            let remainder: BigInt = modulo(&n, &modulus);

            // if remainder is greater than 2^(w-1) - 1
            if remainder > BigInt::from((1 << (w - 1)) - 1) {
                wnaf.push((remainder - &modulus).to_i8().unwrap());
            } else {
                wnaf.push(remainder.to_i8().unwrap());
            }

            n = n - wnaf[i];
        } else {
            wnaf.push(0);
        }

        n >>= 1;
        i += 1;
    }

    wnaf
}

/// Helper function to get a truly random number
/// 
/// # Returns
/// A BigInt representation of the random number
/// 
pub fn entropy() -> BigInt {
    let mut rng: ThreadRng = thread_rng();

    let mut bytes: [u8; 32] = [0u8; 32]; // 32 bytes is 256 bits

    rng.fill(&mut bytes[..]);

    BigInt::from_bytes_be(Sign::Plus, &bytes)
}      

/// Helper function to calculate the modular multiplicative inverse of a number.
/// This function uses the extended euclidean algorithm to calculate the modular multiplicative inverse.
/// 
/// # Arguments
/// * `n` - A reference to a BigInt that is the number to be modded
/// * `b` - A BigInt that is the modulo
/// * `t1` - An optional BigInt that is the first value in the calculation (default is 0)
/// * `t2` - An optional BigInt that is the second value in the calculation (default is 1)
/// 
/// # Returns
/// A BigInt representation of the modular multiplicative inverse
/// 
pub fn modular_multiplicative_inverse(
    n: &BigInt,
    mut b: BigInt,
    t1: Option<BigInt>,
    t2: Option<BigInt>,
) -> BigInt {
    let t1: BigInt = t1.unwrap_or(zero()); // set default value for t1
    let t2: BigInt = t2.unwrap_or(one());// set default value for t2

    if n == &zero() || b == zero() {
        return zero();
    }

    if b < zero() {
        b = modulo(&b, n);
    }

    let q: BigInt = n / &b;
    let r: BigInt = modulo(n, &b);

    let t3: BigInt = t1 - &q * &t2;

    if r == zero() && b != one() {
        return zero();
    }

    if r == zero() {
        t2
    } else {
        modular_multiplicative_inverse(&b, r, Some(t2), Some(t3))
    }
}