use std;

pub fn xor_all_bytes(key: &u8, bytes: &Vec<&u8>) -> Vec<u8> {
    bytes.iter()
        .map(|x| *x ^ key)
        .collect()
}

pub fn repeating_key_xor<'a, I>(key: I, bytes: I) -> Vec<u8>
    where I: Iterator<Item=&'a u8> + std::clone::Clone
{
    bytes.zip(key.cycle())
        .map(|(byte, key_part)| byte ^ key_part)
        .collect()
}

pub fn hamming_distance<'a, I, J>(bytes: I, bytes2: J) -> u32
    where I: Iterator<Item=&'a u8> + std::clone::Clone,
          J: Iterator<Item=&'a u8> + std::clone::Clone,
{
    bytes.zip(bytes2)
        .map(|(byte, byte2)| (byte ^ byte2).count_ones())
        .sum()
}

pub fn from_slice(bytes: &[u8]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..result.len() {
        result[i] = bytes[i];
    }
    result
}

pub fn from_slice_4(bytes: &[u8]) -> [u8; 4] {
    let mut result = [0; 4];
    for i in 0..result.len() {
        result[i] = bytes[i];
    }
    result
}

// b1 is most significant byte, b4 least significant
pub fn to_u32(b1: u8, b2: u8, b3: u8, b4: u8) -> u32 {
    (b4 as u32) + ((b3 as u32) << 8) + ((b2 as u32) << 16) + ((b1 as u32) << 24)
}

mod test {
    use super::repeating_key_xor;
    use super::hamming_distance;
    use string_util::StringUtil;

    #[test]
    fn test_problem_five() {
        let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";

        let actual = repeating_key_xor(key.as_bytes().iter(), text.as_bytes().iter());
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                          a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".hex_to_bytes();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_problem_six_hamming_distance() {
        let text = "this is a test";
        let text2 = "wokka wokka!!!";

        assert_eq!(37, hamming_distance(text.as_bytes().iter(), text2.as_bytes().iter()))
    }
}