extern crate minigrep;

use std::env;
use minigrep::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let search_string = &args[1];
    let filename = &args[2];

    let file_content = minigrep::read_file(filename);

    let has_search_string = minigrep::search(&file_content, search_string);
    print!("{}\n", has_search_string);
}
