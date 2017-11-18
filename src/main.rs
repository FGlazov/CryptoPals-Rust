extern crate base64;

use std::u8;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let two_strings = string_to_len_2_sub_strings(input);

    let x : Vec<u8> = two_strings.iter()
        .map(|c| convert_string_to_u8(c))
        .collect();
    println!("{}", base64::encode(&x));
}

fn string_to_len_2_sub_strings(input : &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut two_strings: Vec<String> = Vec::new();
    {
        let mut counter = 0;
        let mut string_buffer: String = String::new();
        for character in chars {
            string_buffer += &character.to_string();
            counter = counter + 1;
            if counter % 2 == 0 {
                two_strings.push(string_buffer);
                string_buffer = String::new();
            }
        }
    }
    two_strings
}

fn convert_string_to_u8(string : &str) -> u8 {
    match u8::from_str_radix(string, 16) {
        Ok(v) => return v,
        Err(e) => panic!("Error during convert_string_to_u8: {}", e)
    }
}