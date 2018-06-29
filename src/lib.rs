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

pub fn search(file_content: &str, search_string: &str) -> Vec<usize> {
    let result: Vec<_> = file_content.match_indices(search_string).collect();
    let mut matched_indices = Vec::new();

    for i in result {
        matched_indices.push(i.0);
    }

    matched_indices
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

    #[test]
    fn test_search() {
        let filename = "./test-data/test.txt";
        let result = read_file(filename);
        let file_content = result.unwrap();

        let result = search(&file_content, "is");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 5);

        let result = search(&file_content, "Aloy");
        assert_eq!(result.len(), 0);
    }
}
