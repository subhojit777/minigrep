use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(query: &str, filename: &str) -> Config {
        Config { query: query.to_string(), filename: filename.to_string() }
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}

pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut file_content = String::new();

    f.read_to_string(&mut file_content)
        .expect("Something went wrong while reading the file.");

    file_content
}

pub fn search(file_content: &str, search_string: &str) -> bool {
    file_content.contains(search_string)
}

pub fn parse_config(args: &[String]) -> Config {
    Config::new(&args[1], &args[2])
}
