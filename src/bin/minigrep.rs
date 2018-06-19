extern crate minigrep;

use std::env;
use minigrep::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let parameters = minigrep::parse_config(&args);

    let file_content = minigrep::read_file(parameters.1);

    let has_search_string = minigrep::search(&file_content, parameters.0);
    print!("{}\n", has_search_string);
}
