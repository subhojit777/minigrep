use options::*;
use super::{GenError, GenResult};
use super::minigrep_error::*;

/// The necessary configurations for initializing minigrep.
pub struct Config {
    options: Option<Options>,
    query: String,
    filename: String,
}

impl Config {
    /// Initializes a new Config.
    ///
    /// It returns error if incorrect option is passed.
    pub fn new(options: Option<&str>, query: &str, filename: &str) -> GenResult<Config> {
        if options.is_none() {
            return Ok(Config {
                options: None,
                query: query.to_string(),
                filename: filename.to_string(),
            });
        }

        let mut options_struct = Options::new(false, false);

        for ch in options.unwrap().chars() {
            match ch {
                'i' => options_struct.case_sensitive(true),
                'w' => options_struct.exact_match(true),
                _ => return Err(GenError::from(MinigrepError::new("Invalid option given. Allowed options are 'i' and 'w'."))),

            }
        }

        Ok(Config {
            options: Some(options_struct),
            query: query.to_string(),
            filename: filename.to_string(),
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
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_options() {
        let query = "query";
        let filename = "filename";

        let config = Config::new(Some("iw"), query, filename);

        assert_eq!(config.is_ok(), true);

        let config = config.unwrap();

        let options = config.get_options();

        assert_eq!(options.is_some(), true);

        let options = options.unwrap();

        assert_eq!(options.is_case_sensitive(), true);
        assert_eq!(options.is_exact_match(), true);
        assert_eq!(config.get_query(), query);
        assert_eq!(config.get_filename(), filename);
    }

    #[test]
    fn test_config_without_options() {
        let query = "query";
        let filename = "filename";

        let config = Config::new(None, query, filename);

        assert_eq!(config.is_ok(), true);

        let config = config.unwrap();

        let options = config.get_options();

        assert_eq!(options.is_none(), true);

        assert_eq!(config.get_query(), query);
        assert_eq!(config.get_filename(), filename);
    }

    #[test]
    fn test_config_with_invalid_options() {
        let query = "query";
        let filename = "filename";

        let config = Config::new(Some("vi"), query, filename);

        assert_eq!(config.is_err(), true);
    }

    #[test]
    fn test_config_get_options() {
        let query = String::from("query");
        let filename = String::from("filename");

        let config = Config {
            options: Some(Options::new(true, false)),
            query: query,
            filename: filename,
        };

        let options_struct = Options::new(true, false);
        assert_eq!(config.get_options().is_some(), true);
        assert_eq!(config.get_options().unwrap(), &options_struct);
    }

    #[test]
    fn test_config_get_query() {
        let config = Config {
            options: Some(Options::new(true, false)),
            query: String::from("query"),
            filename: String::from("filename"),
        };

        assert_eq!(config.get_query(), "query");
    }

    #[test]
    fn test_config_get_filename() {
        let config = Config {
            options: Some(Options::new(true, false)),
            query: String::from("query"),
            filename: String::from("filename"),
        };

        assert_eq!(config.get_filename(), "filename");
    }
}
