use bitvec::prelude::*;
use crate::{HX, ROUND_CONSTANTS};

/// Function to hash a string using the SHA-256 algorithm
/// 
/// # Arguments
/// * `data` - A string slice that holds the data to be hashed
/// 
/// # Returns
/// * A string that holds the hashed data as a hexadecimal string
/// 
pub fn hash(data: String) -> String {
    let bit_vec = get_processed_data(data);
    let closest_512_multiple: usize = ((bit_vec.len() + 512 - 1) / 512) * 512;

    let k: [u32; 64] = ROUND_CONSTANTS;
    let mut hash_values: [u32; 8] = HX;

    // 512bits chunk loop
    for i in (512..=closest_512_multiple).step_by(512) {
        let mut w: [u32; 64] = get_big_endian_words_from_512bits(&bit_vec[(i - 512)..i]);
        
        // extended first 16 words into next zero-ed indexes
        for j in 16..64 {
            let s0: u32 = right_rotate(w[j-15], 7) ^ right_rotate(w[j-15], 18) ^ (w[j-15] >> 3);
            let s1: u32 = right_rotate(w[j-2], 17) ^ right_rotate(w[j-2], 19) ^ (w[j-2] >> 10);

            w[j] = w[j-16].wrapping_add(s0).wrapping_add(w[j-7]).wrapping_add(s1);
        }
        
        let mut a: u32 = hash_values[0];
        let mut b: u32 = hash_values[1];
        let mut c: u32 = hash_values[2];
        let mut d: u32 = hash_values[3];
        let mut e: u32 = hash_values[4];
        let mut f: u32 = hash_values[5];
        let mut g: u32 = hash_values[6];
        let mut h: u32 = hash_values[7];

        // compress chunk into hash values
        for j in 0..64 {
            #[allow(non_snake_case)]
            let S1: u32 = right_rotate(e, 6) ^ right_rotate(e, 11) ^ right_rotate(e, 25);
            let ch: u32 = (e & f) ^ (!e & g);
            let temp1: u32 = h.wrapping_add(S1).wrapping_add(ch).wrapping_add(k[j]).wrapping_add(w[j]);
            
            #[allow(non_snake_case)]
            let S0: u32 = right_rotate(a, 2) ^ right_rotate(a, 13) ^ right_rotate(a, 22);
            let maj: u32 = (a & b) ^ (a & c) ^ (b & c);
            let temp2: u32 = S0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);

        }

        // update hash values
        hash_values[0] = hash_values[0].wrapping_add(a);
        hash_values[1] = hash_values[1].wrapping_add(b);
        hash_values[2] = hash_values[2].wrapping_add(c);
        hash_values[3] = hash_values[3].wrapping_add(d);
        hash_values[4] = hash_values[4].wrapping_add(e);
        hash_values[5] = hash_values[5].wrapping_add(f);
        hash_values[6] = hash_values[6].wrapping_add(g);
        hash_values[7] = hash_values[7].wrapping_add(h);
    }
    
    return hash_values.iter().map(|&val| format!("{:08x}", val)).collect();
}


// ------------------- Helper functions ------------------- //



/// Function to get the initial hash values for the SHA-256 algorithm.
/// Values are hardcoded now, but they come from this function.
/// 
/// # Returns
/// * An array of 8 u32 values that represent the initial hash values
/// 
pub fn get_initial_hash_values() -> [u32; 8] {
    let mut h: [u32; 8] = [0; 8];

    for (i, p) in get_first_primes(8).iter().enumerate() {
        let root: f64 = (*p as f64).sqrt();
        h[i] = ((root - root.floor()) * (2f64.powi(32))) as u32;
    }

    h
}


/// Function to get the round constants for the SHA-256 algorithm.
/// Values are hardcoded now, but they come from this function.
/// 
/// # Returns
/// * An array of 64 u32 values that represent the round constants
/// 
pub fn get_round_constants() -> [u32; 64] {
    let mut k: [u32; 64] = [0; 64];
    
    for (i, p) in get_first_primes(64).iter().enumerate() {
        let cube_root: f64 = (*p as f64).cbrt();
        k[i] = ((cube_root - cube_root.floor()) * (2f64.powi(32))) as u32;
    }

    k
}

/// Function that checks if a number is prime
/// 
/// # Arguments
/// * `x` - A u32 number to check if it is prime
/// 
/// # Returns
/// * A boolean that is true if the number is prime, false otherwise
/// 
fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }

    // check if x is divisible by any number from 2 to sqrt(x)
    let sqrt_x: u32 = (x as f64).sqrt() as u32;
    (2..=sqrt_x).all(|i: u32| x % i != 0)
}

/// Function to get the first prime numbers up to a limit
/// 
/// # Arguments
/// * `lim` - A usize number that represents the limit of prime numbers to get
/// 
/// # Returns
/// * A vector of u32 values that represent the first prime numbers up to the limit
/// 
fn get_first_primes(lim: usize) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut n: u32 = 2;

    while primes.len() < lim {
        if is_prime(n) {
            primes.push(n);
        }
        n += 1;
    }
    return primes;
} 

/// Function to get the 512 bits chunk as an array of 64 32-bit words.
/// Most words are 0s only 16 first are filled with the 32 bits of the slice.
/// 
/// # Arguments
/// * `slice` - A BitSlice that holds the 512 bits chunk
/// 
/// # Returns
/// * An array of 64 u32 values that represent the 512 bits chunk
/// 
fn get_big_endian_words_from_512bits(slice: &BitSlice) -> [u32; 64] {

    // init schedule array with 0s
    let mut w: [u32; 64] = [0; 64];
    let mut j = 0;

    // iterate over every 32 bits of the slice and add it to the w array (big-endian)
    for i in (32..=slice.len()).step_by(32) {

        // load_le and load_be methods not working here, had to do it by hand
        for (k, bit) in slice[(i-32)..i].iter().enumerate() {
            w[j] |= if *bit { 1 << 31 - k } else { 0 };
        }

        j += 1;
    }

    w
}

/// Function to right rotate a 32-bit number by n bits
/// 
/// # Arguments
/// * `x` - A u32 number to rotate
/// * `n` - A u32 number that represents the number of bits to rotate
/// 
fn right_rotate(x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
}
 
/// Function to get the processed data for the SHA-256 algorithm.
/// The data is processed as follows:
/// 1. Add 1 to the end of the data
/// 2. Add 0s until the length of the data is a multiple of 512
/// 3. Add the number of bits from the original data in big-endian
/// 
/// # Arguments
/// * `data` - A string that holds the data to be processed
/// 
/// # Returns
/// * A BitVec that holds the processed data
///
fn get_processed_data(data: String) -> BitVec {
    let mut bit_vec: BitVec = bitvec![];

    // iterate over every bit of the data and add it to the bitvec
    for c in data.chars() {
        let mut c_as_32: u32 = c as u32;

        for _ in 0..8 { // 8 bits for 0-led chars
            bit_vec.push((1 << 7) & c_as_32 != 0);
            c_as_32 <<= 1;
        }
    }

    bit_vec.push(true); // add one to the end of the bitvec

    let closest_512_multiple: usize = ((bit_vec.len() + 64 + 512 - 1) / 512) * 512;
    
    // 0 padding
    for _ in 0..closest_512_multiple - bit_vec.len() - 64 {
        bit_vec.push(false);
    }

    let mut data_n_bits: u64 = (data.len() * 8) as u64;
    // add number of bits from original data in big-endian
    for _ in 0..64 {
        bit_vec.push(data_n_bits & (1 << 63) != 0);
        data_n_bits <<= 1;
    }
    
    return bit_vec;
}
