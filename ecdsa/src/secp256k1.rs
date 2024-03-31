use num_bigint::BigInt;
use num_traits::{zero, one};


/* Helpers */

// Helper function to handle negative numbers. Ex: -21 % 4 = -1 but in modular arithmetics -21 mod 4 = 3 
fn modulo(x: &BigInt, m: &BigInt) -> BigInt {
    ((x % m) + m) % m
}

pub fn modular_multiplicative_inverse(
    mut n: BigInt,
    mut b: BigInt,
    t1: Option<BigInt>,
    t2: Option<BigInt>,
    s1: Option<BigInt>,
    s2: Option<BigInt>
) -> BigInt {
    let mut t1: BigInt = t1.unwrap_or(one());
    let mut t2: BigInt = t2.unwrap_or(zero());
    let mut s1: BigInt = s1.unwrap_or(zero());
    let mut s2: BigInt = s2.unwrap_or(one());

    let q: BigInt = if n < b { &b / &n } else { &n / &b };
    let r: BigInt = if n < b { modulo(&b, &n) } else { modulo(&n, &b) };

    if n < b {
        b = r;
        s1 -= &t1 * &q;
        s2 -= &t2 * q;
    } else {
        n = r;
        t1 -= &s1 * &q;
        t2 -= &s2 * q;
    }

    if n == one() {
        return t1;
    }

    modular_multiplicative_inverse(n, b, Some(t1), Some(t2), Some(s1), Some(s2))
}
