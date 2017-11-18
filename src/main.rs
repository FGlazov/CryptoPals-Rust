mod byte_util;
mod string_util;
mod xor_cracker;

use string_util::StringUtil;

fn main() {
    let result = xor_cracker::crack_single_key_xor_encryption("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    println!("Key: {} Decoded Text: {} Rating: {}", result.key, result.decoded_text, result.rating)
}