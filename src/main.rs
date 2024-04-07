use ecdsa::secp256k1::Curve;
use num_bigint::BigInt;
use rblock::{Block, Transaction};

pub fn bigint(num: &str) -> BigInt {
    BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
}

fn main() { 
    let p_1 = bigint("78c86580d3b2c9f8392e01f6635356439f706ca200db266ab734504a8bb9553b");
    let p_2 = bigint("552d2967fac5c16573049a4b03b015801688496186873f5a60a7e3bfeeb12570");
    let p_3 = bigint("7d4aec0facb22a7ed640ed207828a9209b4310851ebf04e37a93b3d6d44faa32");
    let p_4 = bigint("d74d10c36094b373ce0e53f7604b830e81f7a95250641d2548d45911bcc8637e");
    let p_5 = bigint("5c6ade1c3fd2c6db9b172037115dbef5c3bf00540f5c53e4df493662f0b13e2e");
    let p_6 = bigint("27b84b5e3a3a37472cd58bbca2269daf78f03cb71251ac7828077bd613bc12e5");

    let curve_1 = Curve::new();
    let point_1 = curve_1.g.multiply(p_1.clone());

    let curve_2 = Curve::new();
    let point_2 = curve_2.g.multiply(p_2.clone());

    let curve_3 = Curve::new();
    let point_3 = curve_3.g.multiply(p_3.clone());

    let curve_4 = Curve::new();
    let point_4 = curve_4.g.multiply(p_4.clone());

    let curve_5 = Curve::new();
    let point_5 = curve_5.g.multiply(p_5.clone());

    let curve_6 = Curve::new();
    let point_6 = curve_6.g.multiply(p_6.clone());

    let t_1 = Transaction::new(&point_1, &point_2, 10.23, &p_1);
    let t_2 = Transaction::new(&point_3, &point_4, 0.24, &p_3);
    let t_3 = Transaction::new(&point_5, &point_6, 5.67, &p_5);

    let g = Block::new_genesis();
    let b = Block::new(&g, &vec![t_1, t_2, t_3]).unwrap();

    print!("{}", b.get_hash());
}