use bitvec::prelude::*;

fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }

    let sqrt_x: u32 = (x as f64).sqrt() as u32;
    (2..=sqrt_x).all(|i: u32| x % i != 0)
}

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

fn get_round_constants() -> [u32; 64] {
    let mut k: [u32; 64] = [0; 64];
    
    for (i, p) in get_first_primes(64).iter().enumerate() {
        let cube_root: f64 = (*p as f64).cbrt();
        k[i] = ((cube_root - cube_root.floor()) * (1 << 31) as f64) as u32;
    }

    k
}

fn get_hash_values() -> [u32; 8] {
    let mut h: [u32; 8] = [0; 8];

    for (i, p) in get_first_primes(8).iter().enumerate() {
        let root: f64 = (*p as f64).sqrt();
        h[i] = ((root - root.floor()) * (1 << 31) as f64) as u32;
    }

    h
}

fn get_big_endian_words_from_512bits(slice: &BitSlice) -> [u32; 64] {
    let mut w: [u32; 64] = [0; 64];
    let mut j = 0;
    for i in (32..=slice.len()).step_by(32) {
        // load_le and load_be not working here, had to do it by hand
        for (k, bit) in slice[(i-32)..i].iter().enumerate() {
            w[j] |= if *bit { 1 << 31 - k } else { 0 };
        }
        j += 1;
    }

    w
}

fn right_rotate(x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
}
 
pub fn hash(data: String) -> String {
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

    let closest_512_multiple: usize = ((bit_vec.len() + 512 - 1) / 512) * 512;

    // 0 padding
    for _ in 0..(closest_512_multiple - bit_vec.len() - 64) {
        bit_vec.push(false); 
    }

    let mut data_n_bits: u64 = (data.len() * 8) as u64;
    // big-endian
    for _ in 0..64 {
        bit_vec.push(data_n_bits & (1 << 63) != 0);
        data_n_bits <<= 1;
    }

    let round_constants: [u32; 64] = get_round_constants();
    let hash_values: [u32; 8] = get_hash_values();

    // chunk loop
    for i in (512..=closest_512_multiple).step_by(512) {
        let mut w: [u32; 64] = get_big_endian_words_from_512bits(&bit_vec[(i - 512)..i]);
        
        // extended first 16 words into next zero-ed indexes
        for j in 16..64 {
            let s0: u32 = right_rotate(w[j-15], 7) ^ right_rotate(w[j-15], 18) ^ (w[j-15] >> 3);
            let s1: u32 = right_rotate(w[j-2], 17) ^ right_rotate(w[j-2], 19) ^ (w[j-2] >> 10);

            w[j] = w[j-16] + s0 + w[j-7] + s1;
        }
        

    }

    return String::from((data.len() * 8).to_string());
}
