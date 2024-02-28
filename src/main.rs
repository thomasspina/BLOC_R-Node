extern crate sha256;

use crate::sha256::hash;

fn main() {
    println!("\n{}", hash(String::from("hello world")));
}
