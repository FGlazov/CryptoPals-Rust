use std::collections::HashSet;

pub fn is_ecb_aes_ciphertext(bytes: &Vec<u8>) -> bool {
    let mut blocks = HashSet::with_capacity(bytes.len() / 16);
    for i in 0..bytes.len() / 16 {
        if !blocks.insert(&bytes[i * 16 .. (i + 1) * 16]) {
            return true;
        }
    }
    false
}

mod test {
    use super::is_ecb_aes_ciphertext;

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


}