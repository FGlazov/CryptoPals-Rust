use byte_util;
use aes::sub_bytes;

const RCON_TABLE: [u8; 256] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a,
    0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39,
    0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a,
    0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8,
    0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef,
    0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc,
    0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b,
    0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3,
    0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94,
    0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20,
    0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35,
    0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f,
    0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04,
    0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63,
    0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd,
    0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d
];

fn key_expansion(key: &[u8]) -> [u8; 176] {
    let mut result: [u8; 176] = [0; 176];

    // TODO: This can probably be done natively
    let mut i: usize = 0;
    while i < key.len() {
        // key.len() = 16
        result[i] = key[i];
        i = i + 1;
    }

    i = 4;
    while i < 44 {
        let word_4_before = byte_util::to_u32(result[4 * (i - 4)],
                                              result[4 * (i - 4) + 1],
                                              result[4 * (i - 4) + 2],
                                              result[4 * (i - 4) + 3]);

        let mut temp = byte_util::to_u32(result[4 * (i - 1)],
                                         result[4 * (i - 1) + 1],
                                         result[4 * (i - 1) + 2],
                                         result[4 * (i - 1) + 3]);

        if i % 4 == 0 {
            temp = sub_word(rot_word(temp));
            let rcon = ((RCON_TABLE[i / 4] as u32) << 24);
            temp = temp ^ rcon;
        }


        let new_word = word_4_before ^ temp;
        let new_bytes = bytes(new_word);

        for j in 0..4 {
            result[4 * i + j] = new_bytes[j];
        }

        i = i + 1;
    }

    result
}

fn rot_word(word: u32) -> u32 {
    word.rotate_left(8)
}

fn sub_word(word: u32) -> u32 {
    let bytes = bytes(word);

    let mut b = byte_util::from_slice_4(&bytes);
    sub_bytes::sub_bytes(&mut b);
    byte_util::to_u32(b[0], b[1], b[2], b[3])
}

// TODO: Move this to byte utils maybe and replace other usages.
fn bytes(word: u32) -> [u8; 4] {
    let bytes = [
        ((word >> 24) & 0xFF) as u8,
        ((word >> 16) & 0xFF) as u8,
        ((word >> 8) & 0xFF) as u8,
        (word & 0xFF) as u8,
    ];
    bytes
}

mod test {
    use string_util::StringUtil;
    use byte_util;

    #[test]
    fn test_inv_mix_columns() {
        let key = "2b7e151628aed2a6abf7158809cf4f3c".hex_to_bytes();
        let key_array = byte_util::from_slice(key.as_slice());

        let expanded = super::key_expansion(key.as_slice());
        let actual = expanded.to_vec();

        for byte in actual.iter() {
            print!("{:02X}", byte);
        }

        let expected = "2B7E151628AED2A6ABF7158809CF4F3CA0FAFE1788542CB123A339392A6C7605F2\
        C295F27A96B9435935807A7359F67F3D80477D4716FE3E1E237E446D7A883BEF44A541A8525B7FB671253BDB0BAD\
        00D4D1C6F87C839D87CAF2B8BC11F915BC6D88A37A110B3EFDDBF98641CA0093FD4E54F70E5F5FC9F384A64FB24E\
        A6DC4FEAD27321B58DBAD2312BF5607F8D292FAC7766F319FADC2128D12941575C006ED014F9A8C9EE2589E13F0C\
        C8B6630CA6".hex_to_bytes();

        assert_eq!(expected, actual);
    }
}