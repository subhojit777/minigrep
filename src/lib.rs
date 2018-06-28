use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub struct Config {
    query: String,
    filename: String
}

type ReadFileResult <T> = Result<T, Error>;

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

pub fn read_file(filename: &str) -> ReadFileResult<String> {
    let mut f = File::open(filename)?;
    let mut file_content = String::new();

    f.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn search(file_content: &str, search_string: &str) -> bool {
    file_content.contains(search_string)
}

pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("too few arguments");
    }

    Ok(Config::new(&args[1], &args[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let query = "query";
        let filename = "filename";

        let config = Config::new(query, filename);

        assert_eq!(config.get_query(), query);
        assert_eq!(config.get_filename(), filename);
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
}
