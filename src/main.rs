mod string_util;

use string_util::StringUtil;

fn main() {
    let result = "1c0111001f010100061a024b53535009181c"
        .hex_xor("686974207468652062756c6c277320657965");
    println!("{}", result)
}