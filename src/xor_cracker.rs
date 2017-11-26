use std;
use byte_util;
use string_util::StringUtil;
use std::ascii::AsciiExt;

pub fn detect_single_byte_xor_encryption<I>(candidates_hex: I) -> Vec<SingleKeyDecryptionResult>
    where I: Iterator<Item=String>
{
    candidates_hex
        .map(|x| crack_single_byte_xor_encryption(x.as_str()))
        .filter(|x| x.rating > 0)
        .collect()
}

pub fn crack_single_byte_xor_encryption(hex_ciphertext: &str) -> SingleKeyDecryptionResult {
    let bytes_ciphertext = hex_ciphertext.hex_to_bytes();

    let mut result = SingleKeyDecryptionResult {
        key: 0,
        decoded_text: String::new(),
        rating: std::i32::MIN,
    };
    for i in 0..256 {
        let key = i as u8;
        let decoded_bytes = byte_util::xor_all_bytes(&key, &bytes_ciphertext);
        let decoded_text = match String::from_utf8(decoded_bytes) {
            Ok(s) => s,
            Err(_) => {
                // Case where xoring creates invalid utf-8. Can't be the key then.
                continue
            }
        };
        let rating = create_rating(&decoded_text);

        if rating > result.rating {
            result = SingleKeyDecryptionResult {
                key,
                decoded_text,
                rating
            }
        }
    }

    result
}

// Based on letter frequency in english.
// See https://en.wikipedia.org/wiki/Letter_frequency
fn create_rating(candidate_decoded_text: &String) -> i32 {
    let very_common_characters = "eta "; // Note this contains a space
    let common_characters = "oinshr ";
    let somewhat_common_characters = "dl\n";
    let rest_of_alphabet = "cumwfgypbvk";
    let uncommon_characters = "vkjxqz";

    // Heuristic - this algorithm could likely be improved.
    let mut rating: i32 = 0;
    for character in candidate_decoded_text.chars() {
        if character.is_uppercase() {
            rating -= 4;
        }

        let character = character.to_ascii_lowercase();
        if very_common_characters.contains(character) {
            rating += 4;
        } else if common_characters.contains(character) {
            rating += 2;
        } else if somewhat_common_characters.contains(character) {
            rating += 1;
        } else if character.is_numeric() || uncommon_characters.contains(character) {
            rating -= 2;
        } else if !character.is_numeric() && !rest_of_alphabet.contains(character) {
            // Char is not a letter, space, or number - meaning a special character like $ or !
            rating -= 4;
        }
    }

    rating
}

pub struct SingleKeyDecryptionResult {
    pub key: u8,
    pub decoded_text: String,
    pub rating: i32
}

//todo : Do this better somehow (imports for test only)
#[allow(unused_imports)]
use std::io::BufRead;
#[allow(unused_imports)]
use std::io::BufReader;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::path::PathBuf;

#[test]
fn test_problem_three() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let actual = crack_single_byte_xor_encryption(ciphertext);

    let expected_key: u8 = 88;
    let expected_decoded_text = "Cooking MC's like a pound of bacon";

    assert_eq!(expected_key, actual.key);
    assert_eq!(expected_decoded_text, actual.decoded_text)
}

#[test]
fn test_problem_four() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_resources/4.txt");
    let f = match File::open(d) {
        Ok(a) => { a }
        Err(b) => { panic!(b) }
    };

    let lines = BufReader::new(f).lines()
        .map(|x| x.unwrap());

    let decrypted_results = detect_single_byte_xor_encryption(lines);

    assert_eq!(1, decrypted_results.len());
    let result: &SingleKeyDecryptionResult = decrypted_results.get(0).unwrap();
    assert_eq!("Now that the party is jumping\n", result.decoded_text)
}