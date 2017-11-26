extern crate base64;

use std::u8;

pub trait StringUtil {
    fn hex_to_bytes(&self) -> Vec<u8>;
    fn hex_to_base64(&self) -> String;
    fn hex_to_utf8_string(&self) -> String;
    fn hex_xor(&self, other: &str) -> String;
}

// TODO: Maybe create "Hex" wrapper for string?
impl StringUtil for str {
    fn hex_to_bytes(&self) -> Vec<u8> {
        let two_strings = string_to_len_2_slices(self);

        two_strings.iter()
            .map(|c| convert_string_to_u8(c))
            .collect()
    }

    fn hex_to_base64(&self) -> String {
        base64::encode(&self.hex_to_bytes())
    }

    fn hex_to_utf8_string(&self) -> String {
        match String::from_utf8(self.hex_to_bytes()) {
            Ok(s) => return s,
            Err(e) => panic!("{} is not a hex string, error: {}", self, e)
        }
    }

    fn hex_xor(&self, other: &str) -> String {
        assert_eq!(self.len(), other.len());

        let bytes_self = self.hex_to_bytes();
        let bytes_other = other.hex_to_bytes();

        let result: Vec<String> = bytes_self.iter()
            .zip(bytes_other)
            .map(|(x, y)| x ^ y)
            .map(|x| to_hex(&x))
            .collect();

        result.join("")
    }
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
        Err(e) => panic!("Error during convert_string_to_u8 for {}, error message: {}", string, e)
    }
}

fn to_hex(byte: &u8) -> String {
    let numbers = "0123456789abcdef";

    let upper_half = byte >> 4;
    let lower_half = byte - (upper_half << 4);

    let string_upper_half = numbers[upper_half as usize..upper_half as usize + 1].to_string();
    let string_lower_half = numbers[lower_half as usize..lower_half as usize + 1].to_string();

    [string_upper_half, string_lower_half].join("")
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_problem_one() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(expected, input.hex_to_base64());
    }

    #[test]
    fn test_problem_two() {
        let actual = "1c0111001f010100061a024b53535009181c"
            .hex_xor("686974207468652062756c6c277320657965");
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_convert_to_utf8() {
        let expected = "the smart fox jumped over the lazy dog";

        let mut hex_chars: Vec<String> = Vec::new();
        let bytes = expected.as_bytes();
        for byte in bytes {
            hex_chars.push(to_hex(byte));
        }
        let hexed_string: String = hex_chars.join("");
        assert_eq!(expected, hexed_string.hex_to_utf8_string());
    }
}