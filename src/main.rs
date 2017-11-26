mod byte_util;
mod string_util;
mod xor_cracker;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = match File::open("4.txt") {
        Ok(a) => { a }
        Err(b) => { panic!(b) }
    };

    let lines = BufReader::new(f).lines()
        .map(|x| x.unwrap());

    let decrypted_results= xor_cracker::detect_single_byte_xor_encryption(lines);
    for result in decrypted_results {
        println!("Result: {} Rating: {}", result.decoded_text, result.rating);
    }
}