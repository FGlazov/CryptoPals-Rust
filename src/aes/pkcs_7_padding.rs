pub fn pad_to_16(text: &mut Vec<u8>) {
    let padding_length = 16 - (text.len() % 16);
    for _ in 0..padding_length {
        text.push(padding_length as u8);
    }
}

pub fn depad_from_16(text: &mut Vec<u8>) {
    let length = {
        text.len()
    };
    let last = {
        *(text.last().unwrap())
    };

    text.truncate(length - last as usize)
}


mod test {
    use super::pad_to_16;
    use super::depad_from_16;
    use string_util::StringUtil;

    #[test]
    fn test_pad_to_16() {
        let mut input = "123456".hex_to_bytes();
        pad_to_16(&mut input);
        let expected = vec![18, 52, 86, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13];

        assert_eq!(input, expected);
    }

    #[test]
    fn test_pad_to_16_exact() {
        let mut input = "12345678901234567890123456789012".hex_to_bytes();
        pad_to_16(&mut input);
        let expected = vec![18, 52, 86, 120, 144, 18, 52, 86, 120, 144, 18, 52, 86, 120,
                            144, 18, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16];

        assert_eq!(input, expected)
    }


    #[test]
    fn test_depad_from_16() {
        let mut input = vec![18, 52, 86, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13];
        let expected = vec![18, 52, 86];
        depad_from_16(&mut input);

        assert_eq!(input, expected);
    }
}