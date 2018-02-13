use std::collections::HashSet;

pub fn is_ecb_aes_ciphertext(bytes: &Vec<u8>) -> bool {
    let mut blocks = HashSet::with_capacity(bytes.len() / 16);
    for i in 0..bytes.len() / 16 {
        if !blocks.insert(&bytes[i * 16..(i + 1) * 16]) {
            return true;
        }
    }
    false
}

pub fn get_block_cipher_detect_string() -> String {
    "A".repeat(1024)
}

mod test {
    use super::is_ecb_aes_ciphertext;
    use super::super::aes;
    use rand;
    use rand::Rng;
    use rand::distributions::Range;
    use rand::distributions::IndependentSample;

    use std::io::BufRead;
    use std::io::BufReader;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    pub fn test_problem_eight() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/8.txt");
        let f = match File::open(d) {
            Ok(a) => { a }
            Err(b) => { panic!(b) }
        };

        let nr_ecb_encrypted = BufReader::new(f).lines()
            .map(|x| x.unwrap())
            .map(|x| x.into_bytes())
            .filter(|x| is_ecb_aes_ciphertext(x))
            .count();

        assert_eq!(1, nr_ecb_encrypted)
    }

    #[test]
    pub fn test_problem_eleven() {
        let plaintext = super::get_block_cipher_detect_string().into_bytes();
        for _ in 0..100 {
            let (ciphertext, use_ecb) = encrypt_with_ecb_or_cbc(&plaintext);
            assert_eq!(is_ecb_aes_ciphertext(&ciphertext), use_ecb)
        }
    }

    // todo refactor this
    pub fn encrypt_with_ecb_or_cbc(plaintext: &Vec<u8>) -> (Vec<u8>, bool) {
        let use_ecb = rand::random();

        let length_range = Range::new(5, 10);
        let len_prefix = length_range.ind_sample(&mut rand::thread_rng());
        let len_suffix = length_range.ind_sample(&mut rand::thread_rng());

        let prefix: Vec<u8> = rand::thread_rng()
            .gen_iter::<u8>()
            .take(len_prefix)
            .collect();

        let suffix: Vec<u8> = rand::thread_rng()
            .gen_iter::<u8>()
            .take(len_suffix)
            .collect();

        let mut mod_plaintext = Vec::with_capacity(len_prefix + len_suffix + plaintext.len());
        mod_plaintext.extend(prefix);
        mod_plaintext.extend(plaintext);
        mod_plaintext.extend(suffix);

        let key: Vec<u8> = rand::thread_rng()
            .gen_iter::<u8>()
            .take(16)
            .collect();

        let nonce: Vec<u8> = rand::thread_rng()
            .gen_iter::<u8>()
            .take(16)
            .collect();

        let ciphertext = match use_ecb {
            true => aes::encrypt_ecb(mod_plaintext, &key),
            false => aes::encrypt_cbc(mod_plaintext, &key, &nonce)
        };

        (ciphertext, use_ecb)
    }
}