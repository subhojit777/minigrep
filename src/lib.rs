pub mod config;
mod minigrep_error;
pub mod options;
extern crate regex;

use config::*;
use minigrep_error::*;
use regex::RegexBuilder;
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

/// Executes the search.
///
/// It returns the indices where the query is found in the content.
pub fn search(config: &mut Config) -> Vec<usize> {
    // We make sure that the file is always read from the beginning.
    config.get_file_as_mut().seek(SeekFrom::Start(0)).unwrap();

    let mut matched_indices = Vec::new();
    let mut file_content = String::new();
    config.get_file().read_to_string(&mut file_content).unwrap();
    let mut query_copy = config.get_query().to_string();
    let mut regex_builder = RegexBuilder::new(&query_copy);

    match config.get_options() {
        Some(options) => {
            // In case of word boundary check, recreate the regex builder with
            // a word boundary regex.
            if options.is_exact_match() {
                query_copy.clear();
                write!(query_copy, r"\b{}\b", config.get_query()).unwrap();
                regex_builder = RegexBuilder::new(&query_copy);
            }

            if options.is_case_sensitive() {
                regex_builder.case_insensitive(true);
            }
        }
        None => {}
    }

    for mat in regex_builder.build().unwrap().find_iter(&file_content) {
        matched_indices.push(mat.start());
    }

    matched_indices
}

/// Parses the command line arguments and prepares them for usage.
///
/// It returns error in following cases:
/// - Too few arguments are passed.
/// - Incorrect options are passed.
/// - File could not be opened.
pub fn parse_config(args: &[String]) -> GenResult<Config> {
    let args_length = args.len();
    let filename = &args[args_length - 1];

    if args_length < 3 {
        return Err(GenError::from(MinigrepError::new("too few arguments")));
    }

    let file = File::open(filename);
    if file.is_err() {
        return Err(GenError::from(file.err().unwrap()));
    }

    let file = file.unwrap();
    if !args[1].starts_with("-") {
        return Ok(Config::new(None, &args[1], &file)?);
    }

    Ok(Config::new(
        Some(&args[1].trim_left_matches('-')),
        &args[2],
        &file,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let f = File::open("./test-data/test.txt").unwrap();
        let mut config = Config::new(None, "is", &f).unwrap();
        let result = search(&mut config);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 5);

        let mut config = Config::new(Some("iw"), "is", &f).unwrap();
        let result = search(&mut config);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 5);

        let mut config = Config::new(Some("i"), "TEST", &f).unwrap();
        let result = search(&mut config);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 8);

        let mut config = Config::new(None, "Aloy", &f).unwrap();
        let result = search(&mut config);

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
