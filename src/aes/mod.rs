pub mod sub_bytes;
mod shift_rows;
mod mix_columns;
mod add_round_key;
mod key_expansion;

pub fn encrypt_exact(plaintext: &[u8], key: &[u8]) -> [u8; 16] {
    let key_schedule = key_expansion::key_expansion(key);
    let mut state = [0; 16];
    state.clone_from_slice(plaintext);

    add_round_key::add_round_key(&mut state, &key_schedule[0..4]);

    for round in 1..10 {
        sub_bytes::sub_bytes(&mut state);
        shift_rows::shift_rows(&mut state);
        mix_columns::mix_columns(&mut state);
        add_round_key::add_round_key(&mut state, &key_schedule[4 * round..4 * (round + 1)])
    }

    sub_bytes::sub_bytes(&mut state);
    shift_rows::shift_rows(&mut state);
    add_round_key::add_round_key(&mut state, &key_schedule[40..44]);

    state
}

mod test {
    use string_util::StringUtil;
    use byte_util;

    #[test]
    fn test_inv_mix_columns() {
        let input = "00112233445566778899aabbccddeeff".hex_to_bytes();
        let key = "000102030405060708090a0b0c0d0e0f".hex_to_bytes();
        let actual = super::encrypt_exact(input.as_slice(), key.as_slice()).to_vec();

        let expected = "69c4e0d86a7b0430d8cdb78070b4c55a".hex_to_bytes();
        assert_eq!(expected, actual);
    }
}