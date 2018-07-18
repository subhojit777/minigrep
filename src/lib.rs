use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

type ReadFileResult <T> = Result<T, Error>;

/// The necessary configurations for initializing minigrep.
pub struct Config {
    options: String,
    query: String,
    filename: String
}

pub enum Options {
    CaseSensitive,
    ExactMatch
}

impl Config {
    /// Initializes a new Config.
    ///
    /// It returns error if incorrect option is passed.
    pub fn new(options: &str, query: &str, filename: &str) -> Result<Config, &'static str> {
        if !options.starts_with("-") {
            return Err("Options should start with -");
        }

        Ok(Config { options: options[1..].to_string(), query: query.to_string(), filename: filename.to_string() })
    }

    /// Returns options for a Config.
    pub fn get_options(&self) -> &str {
        &self.options
    }

    /// Returns query for a Config.
    pub fn get_query(&self) -> &str {
        &self.query
    }

    /// Returns filename for a config.
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}

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
pub fn search(file_content: &str, search_string: &str, options: &str) -> Vec<usize> {
    let mut matched_indices = Vec::new();
    let mut file_content_copy = file_content.to_string();
    let mut search_string_copy = search_string.to_string();

    // TODO: Replace the code below by https://docs.rs/regex/1.0.1/regex/struct.RegexBuilder.html#method.new
    if options == "i" {
        file_content_copy = file_content_copy.to_lowercase();
        search_string_copy = search_string_copy.to_lowercase();
    }

    // TODO: https://gist.github.com/rust-play/54c6bbfce4e3fd40d02c7a236487696b
    let result: Vec<_> = file_content_copy.match_indices(&search_string_copy).collect();
    for i in result {
        matched_indices.push(i.0);
    }

    matched_indices
}

/// Parses the command line arguments and prepares them for usage.
///
/// It returns error if too few arguments are passed.
pub fn parse_config(args: &[String]) -> Result<Config, &str> {
    // TODO: Make `options` optional.
    if args.len() < 4 {
        return Err("too few arguments");
    }

    Ok(Config::new(&args[1], &args[2], &args[3])?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let options = "-i";
        let query = "query";
        let filename = "filename";

        let config = Config::new(options, query, filename);

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.get_options(), &options[1..]);
        assert_eq!(config.get_query(), query);
        assert_eq!(config.get_filename(), filename);

        let options = "-";

        let config = Config::new(options, query, filename);

        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.get_options().is_empty());
        assert_eq!(config.get_query(), query);
        assert_eq!(config.get_filename(), filename);

        let options = "";

        let config = Config::new(options, query, filename);

        assert!(config.is_err());
    }

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

        let _ = f.unwrap().read_to_string(&mut file_content);

        let result = search(&file_content, "is", "");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 5);

        let result = search(&file_content, "TEST", "i");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 8);

        let result = search(&file_content, "Aloy", "");
        assert_eq!(result.len(), 0);
    }
}
