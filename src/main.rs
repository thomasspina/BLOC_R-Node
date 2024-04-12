// TODO: add wallet capabilities
// TODO: add p2p networking
    // TODO: add miner nodes


use ecdsa::secp256k1::{get_curve_computed_points, Curve, W};
use num_bigint::BigInt;
use rblock::{Block, Transaction};
use serde_json;

fn bigint(num: &str) -> BigInt {
    BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
}

// testing to see if blockchain works correctly
fn main() { 
    let p_1 = bigint("78c86580d3b2c9f8392e01f6635356439f706ca200db266ab734504a8bb9553b");
    let p_2 = bigint("552d2967fac5c16573049a4b03b015801688496186873f5a60a7e3bfeeb12570");

    let curve_1 = Curve::new();
    let point_1 = curve_1.g.multiply(p_1.clone(), W, get_curve_computed_points());

    let curve_2 = Curve::new();
    let point_2 = curve_2.g.multiply(p_2.clone(), W, get_curve_computed_points());

    let t_1 = Transaction::new(&point_1, &point_2, 4.23, &p_1);

    let b = Block::new(&Block::new_genesis(), &vec![t_1]);
    b.store_block();

    let _k = serde_json::to_string(&b).unwrap();
}