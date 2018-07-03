extern crate minigrep;
extern crate colored;

use std::env;
use std::process;
use minigrep::*;
use colored::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = minigrep::parse_config(&args).unwrap_or_else(|err| {
        println!("Error occurred while processing: {}", err);
        process::exit(1);
    });

    let file_content: String = minigrep::read_file(config.get_filename()).expect("Something went wrong.");
    let matched_indices = minigrep::search(&file_content, config.get_query());

    // No need to proceed if no match is found.
    if matched_indices.len() == 0 {
        process::exit(0);
    }

    // Print the file content, and highlight the query.
    let query_length = config.get_query().len();
    let mut highlight_query_counter = 0;
    let file_content_char_indices = file_content.char_indices();
    let matched_indices_as_iter = matched_indices.iter();
    let mut matched_index = matched_indices_as_iter.next().unwrap();

    for i in file_content_char_indices {
        if i.0 == *matched_index || highlight_query_counter < query_length {
            print!("{}", (i.1).green().bold());
            highlight_query_counter += 1;
            matched_index = matched_indices_as_iter.next().unwrap();
        } else {
            highlight_query_counter = 0;
            print!("{}", i.1);
        }
    }
}
