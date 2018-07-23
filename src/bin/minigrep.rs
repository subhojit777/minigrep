extern crate colored;
extern crate minigrep;

use colored::*;
use minigrep::*;
use std::env;
use std::process;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let config: config::Config = parse_config(&args).unwrap_or_else(|err| {
        println!("Error occurred while processing: {}", err);
        process::exit(1);
    });

    let file_content: String = read_file(config.get_filename()).expect("Something went wrong.");
    let matched_indices = search(&file_content, config.get_query(), config.get_options());

    // No need to proceed if no match is found.
    if matched_indices.len() == 0 {
        process::exit(0);
    }

    // TODO: Find a way to move the code below inside the main module.
    // Print the content and highlight the query string within it.
    let mut start = 0;
    let query_length: usize = config.get_query().len();
    let mut matched_indices_as_iter = matched_indices.iter();

    loop {
        let matched_index = matched_indices_as_iter.next();

        if matched_index.is_none() {
            let normal = &file_content[start..];
            print!("{}", normal);

            break;
        }

        let matched_index = matched_index.unwrap();

        let normal = &file_content[start..*matched_index];
        let highlight_end_pos = matched_index + query_length;
        let highlight = &file_content[*matched_index..highlight_end_pos];

        print!("{}", normal);
        print!("{}", highlight.green().bold());

        start = highlight_end_pos;
    }
}
