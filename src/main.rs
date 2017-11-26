mod byte_util;
mod string_util;
mod xor_cracker;

use xor_cracker::repeating_xor_cracker;

fn main() {
    let key_sizes = repeating_xor_cracker::guess_key_size(&"badsdgdfashdfshjdsnjdagfgsvxc\
     vasdgdasgdasogdfabgjadsdfsafasfdsafasgsagdasfhgfktglhljhjljxbjxoibcjgjtoiewoiotiadgfgoasigjxokicvj\
     xklcbvjxklbjxklsbjoiartgud9iaewqugjasgdpoajgbxpaicgjgjdpasgjoaisgjfjjoasidgfjoigjasgjdasg\
     jaisgjagjpoasg".bytes().collect());

    for key_size in key_sizes {
        println!("{}", key_size)
    }
}