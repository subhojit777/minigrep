extern crate minigrep;

use std::env;
use std::process;
use minigrep::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = minigrep::parse_config(&args).unwrap_or_else(|err| {
        println!("Error occurred while processing: {}", err);
        process::exit(1);
    });

    let file_content: String = minigrep::read_file(config.get_filename()).expect("Something went wrong.");

    let has_search_string: bool = minigrep::search(&file_content, config.get_query());
    print!("{}\n", has_search_string);
}
