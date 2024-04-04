use core::fmt;

use num_bigint::BigInt;
use num_traits::{zero, One, Zero};
use crate::math::{modulo, modular_multiplicative_inverse, mod_sqrt};

use super::Curve;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
    pub fp: BigInt // prime field
}

// adds to_string for Signature struct
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{};y{};fp{}", self.x, self.y, self.fp)
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
            y: zero(),
            fp: self.fp.clone()
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
        let lambda: BigInt = modulo(&(3 * &self.x * &self.x 
            * modular_multiplicative_inverse(&self.fp, 2 * &self.y, None, None)), 
            &self.fp);
        let rx: BigInt = modulo(&(&lambda * &lambda - &self.x - &self.x), &self.fp);
        let ry: BigInt = modulo(&(lambda * (&self.x - &rx) - &self.y), &self.fp);

        Point {
            x: rx,
            y: ry,
            fp: self.fp.clone()
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
            let lambda: BigInt = modulo(
                &((&other.y - &self.y) 
                * modular_multiplicative_inverse(&self.fp, &other.x - &self.x, None, None)
            ), &self.fp);
            let rx: BigInt = modulo(&(&lambda * &lambda - &other.x - &self.x), &self.fp);
            let ry: BigInt = modulo(&(lambda * (&self.x - &rx) - &self.y), &self.fp);

            Point {
                x: rx,
                y: ry,
                fp: self.fp
            }
        }
    }
}

/*
    returns the x coordinate as a compressed point (essentially the public key)
*/
pub fn compress_point(point: Point) -> String {
    let mut prefix: String;

    if &point.y % 2 != zero() {
        prefix = String::from("03");
    } else {
        prefix = String::from("02");
    }

    let hex_point: String = format!("{:x}", point.x);

    if hex_point.len() < 64 {
        prefix.push_str("0");
    }
    prefix.push_str(&hex_point);

    prefix
}

/*
    returns the Point object from the compressed point 
*/
pub fn decompress_point(point: String) -> Point {
    let curve: Curve = Curve::new();

    let prefix: &str = &point[0..2];
    let x_hex: &str = &point[2..];
    let x: BigInt = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();

    // secp256k1 is y^2 = x^3 + 7
    let y_2: BigInt = &x.pow(3) + &BigInt::from(7);

    let y: BigInt = mod_sqrt(y_2, &curve.g.fp).unwrap();

    let y: BigInt = match prefix {
        "02" if &y % 2 == zero() => &curve.g.fp - y,
        "03" if &y % 2 != zero() => &curve.g.fp - y,
        _ => y,
    };

    Point { x, y, fp: curve.g.fp }
}