pub mod sub_bytes;
mod shift_rows;
mod mix_columns;
mod add_round_key;
mod key_expansion;

pub fn encrypt_exact(plaintext: &[u8], key: &[u8]) -> [u8; 16] {
    let key_schedule = key_expansion::key_expansion(key);
    let mut state = [0; 16];
    state.clone_from_slice(plaintext);

    add_round_key::add_round_key(&mut state, &key_schedule[0..16]);

    for round in 1..10 {
        sub_bytes::sub_bytes(&mut state);
        shift_rows::shift_rows(&mut state);
        mix_columns::mix_columns(&mut state);
        add_round_key::add_round_key(&mut state, &key_schedule[16 * round..16 * (round + 1)]);
    }

    sub_bytes::sub_bytes(&mut state);
    shift_rows::shift_rows(&mut state);
    add_round_key::add_round_key(&mut state, &key_schedule[160..176]);

    state
}

pub fn decrypt_exact(ciphertext: &[u8], key: &[u8]) -> [u8; 16] {
    let key_schedule = key_expansion::key_expansion(key);
    let mut state = [0; 16];
    state.clone_from_slice(ciphertext);

    add_round_key::add_round_key(&mut state, &key_schedule[160..176]);

    for round in (1..10).rev() {
        shift_rows::inv_shift_rows(&mut state);
        sub_bytes::inv_sub_bytes(&mut state);
        add_round_key::add_round_key(&mut state, &key_schedule[16 * round..16 * (round + 1)]);
        mix_columns::inv_mix_columns(&mut state);
    }

    shift_rows::inv_shift_rows(&mut state);
    sub_bytes::inv_sub_bytes(&mut state);
    add_round_key::add_round_key(&mut state, &key_schedule[0..16]);

    state
}

pub fn decrypt_ecb(ciphertext: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    for i in 0..ciphertext.len() / 16 {
        let plaintext_block = decrypt_exact(&ciphertext[i*16..i*16 + 16], key);
        plaintext.extend(plaintext_block.iter());
    }

    plaintext
}

mod test {
    use string_util::StringUtil;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::fs::File;
    use std::path::PathBuf;
    use base64;
    use xor_cracker::rating_creator;

    #[test]
    fn test_encrypt_exact() {
        let input = "00112233445566778899aabbccddeeff".hex_to_bytes();
        let key = "000102030405060708090a0b0c0d0e0f".hex_to_bytes();
        let actual = super::encrypt_exact(input.as_slice(), key.as_slice()).to_vec();

        let expected = "69c4e0d86a7b0430d8cdb78070b4c55a".hex_to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_decrypt_exact() {
        let input = "69c4e0d86a7b0430d8cdb78070b4c55a".hex_to_bytes();
        let key = "000102030405060708090a0b0c0d0e0f".hex_to_bytes();
        let actual = super::decrypt_exact(input.as_slice(), key.as_slice()).to_vec();

        let expected = "00112233445566778899aabbccddeeff".hex_to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_problem_seven() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/7.txt");
        let f = match File::open(d) {
            Ok(a) => { a }
            Err(b) => { panic!(b) }
        };
        let lines: Vec<String> = BufReader::new(f).lines()
            .map(|x| x.unwrap()).collect();

        let ciphertext = base64::decode(&lines.join("")).unwrap();
        let actual = super::decrypt_ecb(ciphertext, "YELLOW SUBMARINE".as_bytes());
//        println!("Text: {}", String::from_utf8(actual.clone()).unwrap());

        let rating = rating_creator::create_rating(&String::from_utf8(actual).unwrap());
        assert!(rating > 1000);
    }
}