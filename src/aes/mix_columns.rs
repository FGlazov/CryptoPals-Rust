// TODO: Make this constant time (bit slice)
// Multiply the byte in galois field GF(2^8) by the polynomial x (= byte 00000010)
fn g2(byte: u8) -> u8 {
    let shifted = (byte as u16) << 1;
    if shifted & 0x100 == 0x100 {
        ((shifted as u8) ^ 0x1b)
    } else {
        (shifted as u8)
    }
}

// TODO: Make this constant time (bit slice)
fn gf_mult(factor: u8, byte: u8) -> u8 {
    let mut result = 0;
    let mut current_factor = byte;
    if factor & 1 == 1 {
        result = result ^ byte;
    }

    for i in 1..8 {
        current_factor = g2(current_factor);
        if factor & (1 << i) == (1 << i) {
            result = result ^ current_factor;
        }
    }

    result
}

pub fn mix_columns(state: &mut [u8]) {
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

pub fn inv_mix_columns(state: &mut [u8]) {
    for i in 0..4 {
        let s0: u8 = state[i * 4];
        let s1: u8 = state[i * 4 + 1];
        let s2: u8 = state[i * 4 + 2];
        let s3: u8 = state[i * 4 + 3];

        state[i * 4] = gf_mult(0x0E, s0)
            ^ gf_mult(0x0B, s1)
            ^ gf_mult(0x0D, s2)
            ^ gf_mult(0x09, s3);

        state[i * 4 + 1] = gf_mult(0x09, s0)
            ^ gf_mult(0x0E, s1)
            ^ gf_mult(0x0B, s2)
            ^ gf_mult(0x0D, s3);

        state[i * 4 + 2] = gf_mult(0x0D, s0)
            ^ gf_mult(0x09, s1)
            ^ gf_mult(0x0E, s2)
            ^ gf_mult(0x0B, s3);

        state[i * 4 + 3] = gf_mult(0x0B, s0)
            ^ gf_mult(0x0D, s1)
            ^ gf_mult(0x09, s2)
            ^ gf_mult(0x0E, s3);
    }
}

mod test {
    use super::g2;
    use super::gf_mult;
    use super::mix_columns;
    use super::inv_mix_columns;
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
    fn test_gf_mult() {
        assert_eq!(0xFE, gf_mult(0x13, 0x57));
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

    #[test]
    fn test_inv_mix_columns() {
        let input = "fa636a2825b339c940668a3157244d17".hex_to_bytes();
        let mut actual_array = byte_util::from_slice(input.as_slice());
        inv_mix_columns(&mut actual_array);

        let expected = "fc1fc1f91934c98210fbfb8da340eb21".hex_to_bytes();
        let expected_array = byte_util::from_slice(expected.as_slice());

        assert_eq!(expected_array, actual_array);
    }
}