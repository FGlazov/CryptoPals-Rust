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

        let key: Vec<u8> = partial_results.iter()
            .map(|x| x.key)
            .collect();
        let decoded_bytes = byte_util::repeating_key_xor(key.iter(), ciphertext.iter());
        let decoded_text = match String::from_utf8(decoded_bytes) {
            Ok(s) => s,
            Err(_) => {
                // Case where xoring creates invalid utf-8. Can't be the key then.
                continue
            }
        }; // This can be done quicker by cutting together the smaller texts. This is less buggy/cleaner to read.
        let rating = rating_creator::create_rating(&decoded_text);

        key_size_results.push(RepeatingXorDecryptionResult {
            key,
            decoded_text,
            rating,
        })
    }

    key_size_results.sort_by(|a, b| b.rating.cmp(&a.rating));
    key_size_results.get(0).unwrap().clone()
}

fn guess_key_size(bytes: &Vec<u8>) -> Vec<u8> {
    let mut values = (1..41)
        .map(|x| (x, averaged_hamming_distance(bytes, x)))
        .collect::<Vec<(u8, f64)>>();
    values.sort_by(|&(_, value1), &(_, value2)| value1.partial_cmp(&value2).unwrap());
    values.iter()
        .map(|&(key_size, _)| key_size.clone())
        .take(3)
        .collect()
}

fn averaged_hamming_distance(bytes: &Vec<u8>, n: u8) -> f64 {
    let mut result = 0.0;
    let mut compares = 0.0;

    // This is a bit slow on debug version (~20ms per run).
    // Quick enough on release version. (50ms to run 40 of these on i7 4.0 GHZ)
    // Can still be optimized by not allocating new vec of n bytes for every run
    // Also can be done multi threaded

    for i in 1..50 {
        for j in 1..50 {
            compares = compares + 1.0;

            let ith_bytes = n_bytes(n, i, bytes);
            let jth_bytes = n_bytes(n, j, bytes);

            if ith_bytes.len() != jth_bytes.len() {
                // case where we reached end of text.
                break;
            }

            if i == j { continue; }
            result += byte_util::hamming_distance(ith_bytes.iter(), jth_bytes.iter()) as f64 / n as f64;
        }
    }

    result / compares
}

fn n_bytes(n: u8, offset: u8, bytes: &Vec<u8>) -> Vec<u8> {
    let result_ref = bytes.iter().skip(((offset as u32) * (n as u32)) as usize).take(n as usize);

    let mut result = Vec::new();
    for byte in result_ref {
        result.push(byte.clone());
    }
    result
}