pub mod config;
mod options;
extern crate regex;

use config::*;
use options::*;
use regex::RegexBuilder;
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

type ReadFileResult<T> = Result<T, Error>;

/// Returns the content inside the mentioned file name.
pub fn read_file(filename: &str) -> ReadFileResult<String> {
    let mut f = File::open(filename)?;
    let mut file_content = String::new();

    f.read_to_string(&mut file_content)?;

    Ok(file_content)
}

/// Looks for the query inside the given content.
///
/// It returns the indices where the query is found in the content.
pub fn search(file_content: &str, search_string: &str, options: Option<&Options>) -> Vec<usize> {
    let mut matched_indices = Vec::new();
    let mut search_string_copy = search_string.to_string();
    let mut regex_builder = RegexBuilder::new(&search_string_copy);

    match options {
        Some(options_struct) => {
            if options_struct.is_exact_match() {
                search_string_copy.clear();
                write!(search_string_copy, r"\b{}\b", search_string).unwrap();
                regex_builder = RegexBuilder::new(&search_string_copy);
            }

            if options_struct.is_case_sensitive() {
                regex_builder.case_insensitive(true);
            }
        }
        None => {}
    }

    for mat in regex_builder.build().unwrap().find_iter(file_content) {
        matched_indices.push(mat.start());
    }

    matched_indices
}

/// Parses the command line arguments and prepares them for usage.
///
/// It returns error if too few arguments are passed.
pub fn parse_config(args: &[String]) -> Result<Config, &str> {
    let args_length = args.len();
    let filename = args[args_length - 1];

    if args_length < 3 {
        return Err("too few arguments");
    }

    let mut file = File::open(filename)?;

    if !args[1].starts_with("-") {
        return Ok(Config::new(None, &args[1], &args[2])?);
    }

    Ok(Config::new(
        Some(&args[1].trim_left_matches('-')),
        &args[2],
        &args[3],
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let filename = "./test-data/test.txt";
        let result = read_file(filename);

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "This is test data.\n");

        let filename = "../test-data/does-not-exist.txt";
        let result = read_file(filename);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_search() {
        let f = File::open("./test-data/test.txt");
        let mut file_content = String::new();
        let options = None;

        let _ = f.unwrap().read_to_string(&mut file_content);

        let result = search(&file_content, "is", options);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 5);

        let options_struct = Options::new(true, true);
        let options = Some(&options_struct);
        let result = search(&file_content, "is", options);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 5);

        let options_struct = Options::new(true, false);
        let options = Some(&options_struct);
        let result = search(&file_content, "TEST", options);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 8);

        let options = None;
        let result = search(&file_content, "Aloy", options);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_config() {
        let args = [String::from("command"), String::from("too few arguments")];
        let config = parse_config(&args);

        assert_eq!(config.is_err(), true);

        let args = [
            String::from("command"),
            String::from("query"),
            String::from("../test-data/does-not-exist.txt"),
        ];
        let config = parse_config(&args);

        assert_eq!(config.is_err(), true);

        let args = [
            String::from("command"),
            String::from("query"),
            String::from("./test-data/test.txt"),
        ];
        let config = parse_config(&args);

        assert_eq!(config.is_ok(), true);

        let args = [
            String::from("command"),
            String::from("-i"),
            String::from("query"),
            String::from("./test-data/test.txt"),
        ];
        let config = parse_config(&args);

        assert_eq!(config.is_ok(), true);
    }
}
