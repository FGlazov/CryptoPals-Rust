extern crate base64;

use std::u8;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let two_strings = string_to_len_2_slices(input);

    let x: Vec<u8> = two_strings.iter()
        .map(|c| convert_string_to_u8(c))
        .collect();
    println!("{}", base64::encode(&x));
}

fn string_to_len_2_slices(input: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();

    {
        let mut rest_of_input = input;
        while !rest_of_input.is_empty() {
            let (to_push, next_rest) = rest_of_input.split_at(2);
            rest_of_input = next_rest;
            result.push(to_push);
        }
    }

    result
}

fn convert_string_to_u8(string: &str) -> u8 {
    match u8::from_str_radix(string, 16) {
        Ok(v) => return v,
        Err(e) => panic!("Error during convert_string_to_u8: {}", e)
    }
}