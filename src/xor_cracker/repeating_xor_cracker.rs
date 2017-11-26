use std::collections::HashMap;
use xor_cracker::itertools::Itertools;

use byte_util;

use xor_cracker::rating_creator;
use xor_cracker::single_byte_xor_cracker::crack_single_byte_xor_encryption;

#[derive(Clone)]
pub struct RepeatingXorDecryptionResult {
    pub key: Vec<u8>,
    pub decoded_text: String,
    pub rating: i32
}

pub fn crack_repeating_xor_encryption(ciphertext: &Vec<u8>) -> RepeatingXorDecryptionResult {
    let key_sizes = guess_key_size(ciphertext);

    let mut key_size_results = Vec::new();
    for key_size in key_sizes {
        let mut partial_results = Vec::new();
        for i in 0..key_size {
            let partial_bytes: Vec<&u8> = ciphertext.iter().skip(i as usize).step(key_size as usize).collect();
            partial_results.push(crack_single_byte_xor_encryption(&partial_bytes))
        }

        let key : Vec<u8> = partial_results.iter()
            .map(|x| x.key)
            .collect();
        let decoded_bytes = byte_util::repeating_key_xor(key.iter(), ciphertext.iter());
        let decoded_text = match String::from_utf8(decoded_bytes) {
            Ok(s) => s,
            Err(_) => {
                println!("Continue");
                // Case where xoring creates invalid utf-8. Can't be the key then.
                continue
            }
        };
        let rating = rating_creator::create_rating(&decoded_text);

        key_size_results.push(RepeatingXorDecryptionResult {
            key,
            decoded_text,
            rating,
        })
    }

    key_size_results.sort_by(|a, b| a.rating.cmp(&b.rating));

    key_size_results.get(0).unwrap().clone()
}

pub fn guess_key_size(bytes: &Vec<u8>) -> Vec<u8> {
    let mut key_size_to_hamming = HashMap::new();

    for i in 1..41 {
        let key_size = i as u8;
        key_size_to_hamming.insert(key_size, averaged_hamming_distance(bytes, key_size));
    }

    let mut values: Vec<(&(u8), &f32)> = key_size_to_hamming.iter().collect();
    values.sort_by(|&(_, value1), &(_, value2)| value1.partial_cmp(value2).unwrap());

    values.iter()
        .map(|&(key_size, _)| key_size.clone())
        .take(3)
        .collect()
}

fn averaged_hamming_distance(bytes: &Vec<u8>, n: u8) -> f32 {
    let first_n = bytes.iter().take(n as usize);
    let second_n = bytes.iter().skip(n as usize).take(n as usize);

    (byte_util::hamming_distance(first_n, second_n)) as f32 / n as f32
}