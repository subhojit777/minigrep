extern crate minigrep;

use std::env;
use minigrep::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let parameters: Config = minigrep::parse_config(&args);

    let file_content: String = minigrep::read_file(parameters.get_filename());

    let has_search_string: bool = minigrep::search(&file_content, parameters.get_query());
    print!("{}\n", has_search_string);
}
