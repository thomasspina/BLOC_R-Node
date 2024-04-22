use serde::Serialize;

mod db;
mod network;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use crate::db::BlocksDB;

#[derive(Serialize)]
struct Port(u16);


lazy_static! {
    // declare global rust db with mutex for thread safe mutation
    pub static ref GLOBAL_DB: Mutex<BlocksDB>= {
        match BlocksDB::start_db() {
            Ok(db) => {
                Mutex::new(db)
            },
            Err(e) => {
                // if db cannot be started, panic
                panic!("{e}")
            }
        }
    };
}

#[tokio::main]
async fn main() {
    network::api::start_node(8334).await.unwrap();
}


// use db_api::BlocksDB;

// use ecdsa::secp256k1::{get_curve_precomputed_points, Secp256k1, W};
// use num_bigint::BigInt;
// use rblock::{Block, Transaction};
// fn bigint(num: &str) -> BigInt {
//     BigInt::parse_bytes(num.as_bytes(), 16).unwrap()
// }
// fn main() {
//     let p_1 = bigint("78c86580d3b2c9f8392e01f6635356439f706ca200db266ab734504a8bb9553b");
//     let p_2 = bigint("552d2967fac5c16573049a4b03b015801688496186873f5a60a7e3bfeeb12570");

//     let curve_1 = Secp256k1::new();
//     let point_1 = curve_1.g.multiply(p_1.clone(), W, get_curve_precomputed_points());

//     let curve_2 = Secp256k1::new();
//     let point_2 = curve_2.g.multiply(p_2.clone(), W, get_curve_precomputed_points());

//     let t_1 = Transaction::new(&point_1, &point_2, 4.23, &p_1);

//     let transactions = &vec![t_1];
//     let b: Block = Block::new(&Block::new_genesis(), transactions);

//     let mut db: BlocksDB = BlocksDB::start_db().unwrap();

//     db.put_block(&b).unwrap();
    
//     let b2: Block = db.get_block(b.get_height()).unwrap();

//     println!("{}\n\n{}", b, b2);

// }