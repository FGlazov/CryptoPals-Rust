use xor_cracker::rating_creator;
use string_util;
use std::collections::HashMap;
use byte_util;
use xor_cracker::itertools::Itertools;

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