mod string_util;

use string_util::StringUtil;

fn main() {
    println!("{}", "abcd".hex_to_base64());
}