// TODO: add wallet capabilities
// TODO: add p2p networking
    // TODO: add miner nodes

// so this main is gonna be used as the starter file for the node
// i need to run a db and containerize the application with the db 
// then for thorough testing i need to package all in docker and run it on another computer

// TODO: then build an app that lets you visualize that information on a webbrowser
    // once that app is built you could just deploy a version of it online for people to see and so that 
    // a node is always active

mod db;
use db::BlocksDB;

use ecdsa::secp256k1::{get_curve_precomputed_points, Curve, W};
use num_bigint::BigInt;
use rblock::{Block, Transaction};


fn bigint(num: &str) -> BigInt {
    BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
}
fn main() {
    let p_1 = bigint("78c86580d3b2c9f8392e01f6635356439f706ca200db266ab734504a8bb9553b");
    let p_2 = bigint("552d2967fac5c16573049a4b03b015801688496186873f5a60a7e3bfeeb12570");

    let curve_1 = Curve::new();
    let point_1 = curve_1.g.multiply(p_1.clone(), W, get_curve_precomputed_points());

    let curve_2 = Curve::new();
    let point_2 = curve_2.g.multiply(p_2.clone(), W, get_curve_precomputed_points());

    let t_1 = Transaction::new(&point_1, &point_2, 4.23, &p_1);

    let transactions = &vec![t_1];
    let b: Block = Block::new(&Block::new_genesis(), transactions);

    let mut db = BlocksDB::start_db().unwrap();

    let b2: Block = Block::new(&Block::new_genesis(), &vec![]);

    db.put_block(&b);

    let c: Block = db.read_block(b.get_height()).unwrap();

    println!("{}\n\n{}", b2, c);

}