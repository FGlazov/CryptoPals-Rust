use byte_util;

pub fn shift_rows(state: &mut [u8]) {
    shift_rows_helper(state, false);
}

pub fn inv_shift_rows(state: &mut [u8]) {
    shift_rows_helper(state, true);
}

fn shift_rows_helper(state: &mut [u8], inverted: bool) {
    for i in 0..4 {
        let ith_row = byte_util::to_u32(
            state[i], state[i + 4], state[i + 8], state[i + 12]);

        let rotated: u32 = match inverted {
            true => ith_row.rotate_right((i * 8) as u32),
            false => ith_row.rotate_left((i * 8) as u32)
        };

        for j in 0..4 {
            state[i + j * 4] = ((rotated >> (24 - 8 * j)) & 0xFF) as u8
        }
    }
}

mod test {
    use super::shift_rows;
    use super::inv_shift_rows;
    use string_util::StringUtil;
    use byte_util;

    #[test]
    fn test_shift_rows() {
        let input = "3b59cb73fcd90ee05774222dc067fb68".hex_to_bytes();
        let mut actual_array = byte_util::from_slice(input.as_slice());
        shift_rows(&mut actual_array);

        let expected = "3bd92268fc74fb735767cbe0c0590e2d".hex_to_bytes();
        let expected_array = byte_util::from_slice(expected.as_slice());

        assert_eq!(expected_array, actual_array);
    }

    #[test]
    fn test_inv_shift_rows() {
        let input = "3e1c22c0b6fcbf768da85067f6170495".hex_to_bytes();
        let mut actual_array = byte_util::from_slice(input.as_slice());
        inv_shift_rows(&mut actual_array);

        let expected = "3e175076b61c04678dfc2295f6a8bfc0".hex_to_bytes();
        let expected_array = byte_util::from_slice(expected.as_slice());

        assert_eq!(expected_array, actual_array);
    }
}