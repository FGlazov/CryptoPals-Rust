pub fn xor_all_bytes(key : &u8, bytes : &Vec<u8>) -> Vec<u8> {
    bytes.iter()
        .map(|x| x ^ key)
        .collect()
}