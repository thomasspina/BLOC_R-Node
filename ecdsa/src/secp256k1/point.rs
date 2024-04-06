use core::fmt;
use num_bigint::BigInt;
use num_traits::{zero, One, Zero};
use crate::{math::{modular_multiplicative_inverse, modulo, bigint}, secp256k1::FP};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt
    // fp prime field is now in curve or a constant from mod.rs
}

// adds to_string for Signature struct
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{};y{}", self.x, self.y)
    }
}

impl Point {
    /*
        returns the point multiplied by n. Uses fast binary exponentiation
    */
    pub fn multiply(mut self, mut n: BigInt) -> Point {
        let mut res: Point = Point::identity(&self);


        let mut decoy: Point = Point::identity(&self); // used to counter side channel attacks
        while n > zero() {
            if &n & BigInt::one() != BigInt::zero() {
                res = res.add(&self);
            } else {
                decoy = decoy.add(&self);
            }

            self = self.double();
            n >>= 1;
        }

        res
    }    

    /*
        Returns the identity element of the group
    */
    fn identity(&self) -> Point {
        Point {
            x: zero(),
            y: zero()
        }
    }

    /*
        double implementation doubles a point ie, it adds the point to itself (mod fp) using these formulas
        L = [ (3*X^2) / 2*Y ] mod P
        Xr = [ L^2 - 2*X ] mod P
        Yr = [ L*(X - Xr) - Y ] mod P
    */
    fn double(&self) -> Point {
        // we use the modular multiplicative inverse to not have to divide
        let fp: &BigInt = &bigint(FP);

        let lambda: BigInt = modulo(&(3 * &self.x * &self.x 
            * modular_multiplicative_inverse(fp, 2 * &self.y, None, None)), 
            fp);
        let rx: BigInt = modulo(&(&lambda * &lambda - &self.x - &self.x), fp);
        let ry: BigInt = modulo(&(lambda * (&self.x - &rx) - &self.y), fp);

        Point {
            x: rx,
            y: ry
        }
    }

    /*
        add implementation adds a point to another using following formulas
        L = [ (Y' - Y) / (X' - X) ] mod P
        Xr = [ L^2 - X - X' ] mod P
        Yr = [ L*(X - Xr) - Y ] mod P
    */
    pub fn add(self, other: &Point) -> Point {
        if self.x == other.x && self.y == (&other.y * -1) { // check P2 = -P1, vertical line, thus P1 + P2 = 0
            Point::identity(&self)
        } else if self.x == other.x && self.y == other.y { // P1 == P2, use point doubling
            self.double()
        } else if self.x == zero() && self.y == zero() { // 0 + P2 = P2
            other.clone()
        } else if other.x == zero() && other.y == zero() { // P1 + 0 = P1
            self
        } else {
            let fp: &BigInt = &bigint(FP);
            let lambda: BigInt = modulo(
                &((&other.y - &self.y) 
                * modular_multiplicative_inverse(fp, &other.x - &self.x, None, None)
            ), fp);
            let rx: BigInt = modulo(&(&lambda * &lambda - &other.x - &self.x), fp);
            let ry: BigInt = modulo(&(lambda * (&self.x - &rx) - &self.y), fp);

            Point {
                x: rx,
                y: ry
            }
        }
    }
}