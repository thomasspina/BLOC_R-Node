mod db;

use ecdsa::secp256k1::{get_curve_precomputed_points, Secp256k1, W};
use num_bigint::BigInt;
use rblock::{Block, Transaction};

use crate::db::BlocksDB;

fn bigint(num: &str) -> BigInt {
    BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
}

// TODO: figure out to a way to know when its the first time running the app on user computer 
    // to init genesis block and latest block or to get it from server
    // TODO: should genesis block have transactions with addresses to kickstart the currency?
    // TODO: should chainstate be rebuilt whenever you restart your node? to make sure that everything is alright?

fn main() {
    let mut db: BlocksDB = BlocksDB::start_db().unwrap();



    // let p_1 = bigint("78c86580d3b2c9f8392e01f6635356439f706ca200db266ab734504a8bb9553b");
    // let p_2 = bigint("552d2967fac5c16573049a4b03b015801688496186873f5a60a7e3bfeeb12570");

    // let curve_1 = Secp256k1::new();
    // let point_1 = curve_1.g.multiply(p_1.clone(), W, get_curve_precomputed_points());

    // let curve_2 = Secp256k1::new();
    // let point_2 = curve_2.g.multiply(p_2.clone(), W, get_curve_precomputed_points());

    // let t_1 = Transaction::new(&point_1, &point_2, 1000., &p_1);
    // let t_2 = Transaction::new(&point_2, &point_1, 1000., &p_2);    

    // let transactions = &vec![t_1, t_2];


    //db.init_db(&point_1, &point_2);

    // let latest = db.get_latest_block().unwrap();
    // let mut next = Block::new(&latest, transactions);
    // next.reward_miner(&point_1);

    //db.add_block(&next).unwrap();
    //db.rebuild_chainstate().unwrap();

    // println!("{}", db.get_balance(&point_1).unwrap());
    // println!("{}", db.get_balance(&point_2).unwrap());
}