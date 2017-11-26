use std::ascii::AsciiExt;

// Based on letter frequency in english.
// See https://en.wikipedia.org/wiki/Letter_frequency
pub fn create_rating(candidate_decoded_text: &String) -> i32 {
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