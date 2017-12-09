// Multiply the byte in galois field GF(2^8) by the polynomial x (= byte 00000010)
fn g2(byte: u8) -> u8 {
    let shifted = (byte as u16) << 1;
    if shifted & 0x100 == 0x100 {
        ((shifted as u8) ^ 0x1b)
    } else {
        (shifted as u8)
    }
}

fn mix_columns(state: &mut [u8]) {
    for i in 0..4 {
        let s0: u8 = state[i * 4];
        let s1: u8 = state[i * 4 + 1];
        let s2: u8 = state[i * 4 + 2];
        let s3: u8 = state[i * 4 + 3];

        state[i * 4] = g2(s0)               // x * s0
            ^ g2(s1) ^ s1                   // x * s1 + s1
            ^ s2                            // s2
            ^ s3;                           // s3

        state[i * 4 + 1] = s0               // s0
            ^ g2(s1)                        // x * s1
            ^ g2(s2) ^ s2                   // x * s2 + s2
            ^ s3;                           // s3

        state[i * 4 + 2] = s0               // s0
            ^ s1                            // s1
            ^ g2(s2)                        // x * s2
            ^ g2(s3) ^ s3;                  // x * s3 + s3

        state[i * 4 + 3] = g2(s0) ^ s0     // s0 * x + s0
            ^ s1                           // s1
            ^ s2                           // s2
            ^ g2(s3);                      // s3
    }
}

mod test {
    use super::g2;
    use super::mix_columns;
    use byte_util;
    use string_util::StringUtil;

    #[test]
    fn test_g2() {
        assert_eq!(0xAE, g2(0x57));
        assert_eq!(0x47, g2(0xAE));
        assert_eq!(0x8E, g2(0x47));
        assert_eq!(0x07, g2(0x8E));
    }

    #[test]
    fn test_mix_columns() {
        let input = "3bd92268fc74fb735767cbe0c0590e2d".hex_to_bytes();
        let mut actual_array = byte_util::from_slice(input.as_slice());
        mix_columns(&mut actual_array);

        let expected = "4c9c1e66f771f0762c3f868e534df256".hex_to_bytes();
        let expected_array = byte_util::from_slice(expected.as_slice());

        assert_eq!(expected_array, actual_array);
    }
}