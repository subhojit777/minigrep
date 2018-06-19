use std::fs::File;
use std::io::prelude::*;

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
