extern crate base64;

mod byte_util;
mod string_util;
mod xor_cracker;
mod aes;

#[allow(unused_imports)]
use std::io::BufRead;
#[allow(unused_imports)]
use std::io::BufReader;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::path::PathBuf;

use xor_cracker::repeating_xor_cracker;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_resources/6.txt");
    let f = match File::open(d) {
        Ok(a) => { a }
        Err(b) => { panic!(b) }
    };
    let lines :Vec<String> = BufReader::new(f).lines()
        .map(|x| x.unwrap()).collect();
    let ciphertext = base64::decode(&lines.join("")).unwrap();

    let result = repeating_xor_cracker::crack_repeating_xor_encryption(&ciphertext);
    println!("Rating: {} : Key: {} \n\n{}", result.rating, String::from_utf8(result.key).unwrap(), result.decoded_text);
}