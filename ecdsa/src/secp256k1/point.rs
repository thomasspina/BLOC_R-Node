use core::fmt;
use num_bigint::BigInt;
use num_traits::zero;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::{math::{modular_multiplicative_inverse, modulo, bigint, calculate_wnaf}, secp256k1::FP};
use serde::de::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt
    // fp prime field is now in curve or a constant from mod.rs
}

/*
    adds to_string for Point struct
*/
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}_y{}", self.x, self.y)
    }
}

/*
    implement trait for json serialization
*/
impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer 
    {
        let mut state = serializer.serialize_struct("Point", 2)?;
        // encode bigint as hex
        state.serialize_field("x", &format!("{:x}", &self.x))?; 
        state.serialize_field("y", &format!("{:x}", &self.y))?;
        state.end()
    }
}

/* 
    implement for json deserialization for Signature
*/
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        // need to use intermediary struct with strings because 
        // bigint is not directly serializable and desrializable
        #[derive(serde::Deserialize)]
        struct PointFields {
            x: String,
            y: String
        }

        let fields: PointFields = PointFields::deserialize(deserializer)?;

        Ok(Point {
            x: bigint(&fields.x),
            y: bigint(&fields.y)
        })
    }
}

/*
    adds "==" comparison
*/
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    /*
        returns the point multiplied by n using non-adjacent scalar representation.
        https://en.wikipedia.org/wiki/Non-adjacent_form

        it allows for much less additions and doubling, especially using pre comps
    */
    pub fn multiply(self, n: BigInt, width: u32, pre_comp: &std::vec::Vec<Point>) -> Point {
        let wnaf: Vec<i8> = calculate_wnaf(width, n);

        let mut q: Point = Point::identity();

        let mut i: i32 = (wnaf.len() as i32) - 1;

        while i > -1 {
            q = q.double();

            let n: usize = i as usize;

            if wnaf[n] > 0 {
                let d: i8 = (wnaf[n] - 1) / 2;

                q = q.add(&pre_comp[d as usize]);
            } else if wnaf[n] < 0 {
                let d: i8 = (-wnaf[n] - 1) / 2;

                let z: Point = Point {
                    x: pre_comp[d as usize].x.clone(),
                    y: pre_comp[d as usize].y.clone() * -1
                };

                q = q.add(&z);
            }

            i = i - 1;
        }

        q
    } 

    /*
        Returns the identity element of the group
    */
    pub fn identity() -> Self {
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
            Point::identity()
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

/*
    returns a list of precomputed points of width w from Point Q
*/
pub fn precompute_points(mut q: Point, w: u32) -> std::vec::Vec<Point> {
    let mut p: Vec<Point> = vec![q.clone()];

    q = q.double();

    for j in 1..(1 << (w - 1)) {
        let mut buffer: Point = q.clone();
        buffer = buffer.add(&p[(j - 1) as usize]);
        p.push(buffer);
    }

    p
}