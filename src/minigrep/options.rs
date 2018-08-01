/// Possible options for a Config.
#[derive(Debug)]
pub struct Options {
    case_sensitive: bool,
    exact_match: bool,
}

impl Options {
    /// Initializes a new Options.
    pub fn new(case_sensitive: bool, exact_match: bool) -> Options {
        Options {
            case_sensitive: case_sensitive,
            exact_match: exact_match,
        }
    }

    /// Adds case-sensitive to the options.
    pub fn case_sensitive(&mut self, yes: bool) {
        self.case_sensitive = yes;
    }

    /// Adds exact match to the options.
    pub fn exact_match(&mut self, yes: bool) {
        self.exact_match = yes;
    }

    /// Checks if the options has exact match check.
    ///
    /// # Example:
    ///
    /// ```
    /// use minigrep::minigrep::options::Options;
    ///
    /// let options = Options::new(true, true);
    /// assert!(options.is_exact_match());
    /// ```
    pub fn is_exact_match(&self) -> bool {
        self.exact_match
    }

    /// Checks if the options has case-sensitive check.
    ///
    /// # Example:
    ///
    /// ```
    /// use minigrep::minigrep::options::Options;
    ///
    /// let options = Options::new(false, true);
    /// assert!(!options.is_case_sensitive());
    /// ```
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
}

impl PartialEq for Options {
    fn eq(&self, other: &Options) -> bool {
        self.case_sensitive == other.case_sensitive && self.exact_match == other.exact_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_new() {
        let case_sensitive = true;
        let exact_match = false;
        let options = Options::new(case_sensitive, exact_match);

        assert_eq!(options.case_sensitive, case_sensitive);
        assert_eq!(options.exact_match, exact_match);
    }

    #[test]
    fn test_options_case_sensitive() {
        let mut options = Options {
            case_sensitive: false,
            exact_match: false,
        };
        options.case_sensitive(true);

        assert_eq!(options.case_sensitive, true);
        assert_eq!(options.exact_match, false);
    }

    #[test]
    fn test_options_exact_match() {
        let mut options = Options {
            case_sensitive: false,
            exact_match: false,
        };
        options.exact_match(true);

        assert_eq!(options.exact_match, true);
        assert_eq!(options.case_sensitive, false);
    }

    #[test]
    fn test_options_is_case_sensitive() {
        let options = Options {
            case_sensitive: false,
            exact_match: true,
        };

        assert_eq!(options.is_case_sensitive(), false);
    }

    #[test]
    fn test_options_is_exact_match() {
        let options = Options {
            case_sensitive: true,
            exact_match: false,
        };

        assert_eq!(options.is_exact_match(), false);
    }
}
