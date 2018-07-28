use super::minigrep_error::*;
use super::{GenError, GenResult};
use options::*;
use std::fs::File;

/// The necessary configurations for initializing minigrep.
pub struct Config {
    options: Option<Options>,
    query: String,
    file: File,
}

impl Config {
    /// Initializes a new Config.
    ///
    /// It returns error if incorrect option is passed.
    pub fn new(options: Option<&str>, query: &str, file: &File) -> GenResult<Config> {
        if options.is_none() {
            return Ok(Config {
                options: None,
                query: query.to_string(),
                file: file.try_clone().unwrap(),
            });
        }

        let mut options_struct = Options::new(false, false);

        for ch in options.unwrap().chars() {
            match ch {
                'i' => options_struct.case_sensitive(true),
                'w' => options_struct.exact_match(true),
                _ => {
                    return Err(GenError::from(MinigrepError::new(
                        "Invalid option given. Allowed options are 'i' and 'w'.",
                    )))
                }
            }
        }

        Ok(Config {
            options: Some(options_struct),
            query: query.to_string(),
            file: file.try_clone().unwrap(),
        })
    }

    /// Returns options for a Config.
    pub fn get_options(&self) -> Option<&Options> {
        self.options.as_ref()
    }

    /// Returns query for a Config.
    pub fn get_query(&self) -> &str {
        &self.query
    }

    /// Returns filename for a config.
    pub fn get_file(&self) -> &File {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_config_with_options() {
        let query = "query";
        let file = File::open("./test-data/test.txt").unwrap();
        let config = Config::new(Some("iw"), query, &file);

        assert_eq!(config.is_ok(), true);

        let config = config.unwrap();
        let options = config.get_options();

        assert_eq!(options.is_some(), true);

        let options = options.unwrap();

        assert_eq!(options.is_case_sensitive(), true);
        assert_eq!(options.is_exact_match(), true);
        assert_eq!(config.get_query(), query);
        assert_eq!(
            config.get_file().metadata().unwrap().len(),
            file.metadata().unwrap().len()
        );
    }

    #[test]
    fn test_config_without_options() {
        let query = "query";
        let file = File::open("./test-data/test.txt").unwrap();
        let config = Config::new(None, query, &file);

        assert_eq!(config.is_ok(), true);

        let config = config.unwrap();
        let options = config.get_options();

        assert_eq!(options.is_none(), true);
        assert_eq!(config.get_query(), query);
        assert_eq!(
            config.get_file().metadata().unwrap().len(),
            file.metadata().unwrap().len()
        );
    }

    #[test]
    fn test_config_with_invalid_options() {
        let query = "query";
        let file = File::open("./test-data/test.txt").unwrap();
        let config = Config::new(Some("vi"), query, &file);

        assert_eq!(config.is_err(), true);
    }

    #[test]
    fn test_config_get_options() {
        let query = String::from("query");
        let file = File::open("./test-data/test.txt").unwrap();
        let config = Config {
            options: Some(Options::new(true, false)),
            query: query,
            file: file,
        };
        let options_struct = Options::new(true, false);

        assert_eq!(config.get_options().is_some(), true);
        assert_eq!(config.get_options().unwrap(), &options_struct);
    }

    #[test]
    fn test_config_get_query() {
        let file = File::open("./test-data/test.txt").unwrap();
        let config = Config {
            options: Some(Options::new(true, false)),
            query: String::from("query"),
            file: file,
        };

        assert_eq!(config.get_query(), "query");
    }

    #[test]
    fn test_config_get_file() {
        let file = File::open("./test-data/test.txt").unwrap();
        let config = Config {
            options: Some(Options::new(true, false)),
            query: String::from("query"),
            file: file.try_clone().unwrap(),
        };

        assert_eq!(
            config.get_file().metadata().unwrap().len(),
            file.metadata().unwrap().len()
        );
        assert!(config.get_file().metadata().unwrap().is_file());
        assert_eq!(
            config.get_file().metadata().unwrap().file_type(),
            file.metadata().unwrap().file_type()
        );
    }
}
