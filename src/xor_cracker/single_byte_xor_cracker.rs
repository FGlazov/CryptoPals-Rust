use std;
use byte_util;
use string_util::StringUtil;
use xor_cracker::rating_creator;

pub struct SingleByteXorDecryptionResult {
    pub key: u8,
    pub decoded_text: String,
    pub rating: i32
}

pub fn detect_single_byte_xor_encryption<I>(candidates_hex: I) -> Vec<SingleByteXorDecryptionResult>
    where I: Iterator<Item=String>
{
    candidates_hex
        .map(|x| crack_single_byte_xor_encryption(&deref(&x.as_str().hex_to_bytes())))
        .filter(|x| x.rating > 0)
        .collect()
}

fn deref(refed : &Vec<u8>) -> Vec<&u8> {
    refed.iter()
        .map(|x| x)
        .collect()
}

pub fn crack_single_byte_xor_encryption(ciphertext: &Vec<&u8>) -> SingleByteXorDecryptionResult {
    let mut result = SingleByteXorDecryptionResult {
        key: 0,
        decoded_text: String::new(),
        rating: std::i32::MIN,
    };
    for i in 0..256 {
        let key = i as u8;
        let decoded_bytes = byte_util::xor_all_bytes(&key, &ciphertext);
        let decoded_text = match String::from_utf8(decoded_bytes) {
            Ok(s) => s,
            Err(_) => {
                // Case where xoring creates invalid utf-8. Can't be the key then.
                continue
            }
        };
        let rating = rating_creator::create_rating(&decoded_text);

        if rating > result.rating {
            result = SingleByteXorDecryptionResult {
                key,
                decoded_text,
                rating
            }
        }
    }

    result
}



mod test {
    use super::crack_single_byte_xor_encryption;
    use super::deref;
    use super::detect_single_byte_xor_encryption;
    use super::SingleByteXorDecryptionResult;
    use string_util::StringUtil;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_problem_three() {
        let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let actual = crack_single_byte_xor_encryption(&deref(&ciphertext.hex_to_bytes()));

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
        let result: &SingleByteXorDecryptionResult = decrypted_results.get(0).unwrap();
        assert_eq!("Now that the party is jumping\n", result.decoded_text)
    }
}