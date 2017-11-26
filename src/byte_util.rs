use std;

pub fn xor_all_bytes(key: &u8, bytes: &Vec<u8>) -> Vec<u8> {
    bytes.iter()
        .map(|x| x ^ key)
        .collect()
}

pub fn repeating_key_xor<'a, I>(key: I, bytes: I) -> Vec<u8>
    where I: Iterator<Item=&'a u8> + std::clone::Clone
{
    bytes.zip(key.cycle())
        .map(|(byte, key_part)| byte ^ key_part)
        .collect()
}

//todo : Do this better somehow (imports for test only)
#[allow(unused_imports)]
use string_util;
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