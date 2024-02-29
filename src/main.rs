extern crate sha2;

use sha2::sha256::hash;

fn main() {
    print!("{}", hash(String::from("hello world")));
}
