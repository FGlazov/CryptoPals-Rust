pub fn add_round_key(state: &mut [u8], words: &[u8]) {
    for i in 0..state.len() {
        state[i] = state[i] ^ words[i];
    }
}

mod test {
    use super::add_round_key;
    use string_util::StringUtil;
    use byte_util;

    #[test]
    fn test_shift_rows() {
        let input = "c81677bc9b7ac93b25027992b0261996".hex_to_bytes();
        let mut actual_array = byte_util::from_slice(input.as_slice());
        let round_key = "3caaa3e8a99f9deb50f3af57adf622aa".hex_to_bytes();
        add_round_key(&mut actual_array, round_key.as_slice());

        let expected = "f4bcd45432e554d075f1d6c51dd03b3c".hex_to_bytes();
        let expected_array = byte_util::from_slice(expected.as_slice());

        assert_eq!(expected_array, actual_array);
    }
}
