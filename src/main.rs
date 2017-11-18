extern crate base64;

use std::u8;

trait StringUtil {
    fn hex_to_bytes(&self) -> Vec<u8>;

    fn hex_to_base64(&self) -> String;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_problem_one() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(expected, input.hex_to_base64());
    }
}