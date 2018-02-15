use rand;
use rand::Rng;
use base64;

pub struct AesWrapper {
    key: Vec<u8>
}

impl AesWrapper {
    pub fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        let unknown_string = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFp\
        ciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A\
        /IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
        let unknown_bytes = base64::decode(unknown_string).unwrap();

        let mut to_encrypt = Vec::new();
        to_encrypt.extend(plaintext);
        to_encrypt.extend(unknown_bytes);

        super::encrypt_ecb(to_encrypt, &self.key)
    }

    pub fn new() -> AesWrapper {
        let key: Vec<u8> = rand::thread_rng()
            .gen_iter::<u8>()
            .take(16)
            .collect();

        AesWrapper { key }
    }
}