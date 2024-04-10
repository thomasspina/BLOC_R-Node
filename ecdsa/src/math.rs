use num_bigint::{BigInt, Sign};
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use num_traits::{zero, one};
use num_traits::ToPrimitive;

/*
    Helper function to return a hexadecimal string as a bigint
*/
pub fn bigint(num: &str) -> BigInt {
    BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
}

/*
    Helper function to handle negative numbers with modulus operations.
    Ex: -21 % 4 = -1 but in modular arithmetics -21 mod 4 = 3 
*/
pub fn modulo(x: &BigInt, m: &BigInt) -> BigInt {
    ((x % m) + m) % m
}

/*
    Converts an integer n to a w - width NAF representation for 
    less additions during multiplication algo.
*/
pub fn calculate_wnaf(w: u32, mut n: BigInt) -> Vec<i8> {
    let mut wnaf: Vec<i8> = Vec::new();

    let modulus: BigInt = BigInt::from(1 << w);
    let mut i: usize = 0;

    while n >= one() {
        if &n & &one() == one() {
            let remainder: BigInt = modulo(&n, &modulus);

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

/*
    Helper function to get a random 256bit long BigInt that is cryptographically secure.
*/
pub fn entropy() -> BigInt {
    let mut rng: ThreadRng = thread_rng();

    let mut bytes: [u8; 32] = [0u8; 32]; // 32 bytes is 256 bits

    rng.fill(&mut bytes[..]);

    BigInt::from_bytes_be(Sign::Plus, &bytes)
}      

/*
    Helper function to get the modular multiplicative inverse. 
    This function uses the extended euclidean algorithm.
    Ex: (5 * x) mod 7 = 1 what is x. x here is 3
*/
pub fn modular_multiplicative_inverse(
    n: &BigInt,
    mut b: BigInt,
    t1: Option<BigInt>,
    t2: Option<BigInt>,
) -> BigInt {
    let t1 = t1.unwrap_or(zero()); // set default value for t1
    let t2 = t2.unwrap_or(one());// set default value for t2

    if n == &zero() || b == zero() {
        return zero();
    }

    if b < zero() {
        b = modulo(&b, n);
    }

    let q = n / &b;
    let r = modulo(n, &b);

    let t3 = t1 - &q * &t2;

    if r == zero() && b != one() {
        return zero();
    }

    if r == zero() {
        t2
    } else {
        modular_multiplicative_inverse(&b, r, Some(t2), Some(t3))
    }
}