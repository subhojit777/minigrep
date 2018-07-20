pub struct Options {
    case_sensitive: bool,
    exact_match: bool
}

impl Options {
    pub fn new(case_sensitive: bool, exact_match: bool) -> Options {
        Options {case_sensitive: case_sensitive, exact_match: exact_match}
    }

    pub fn case_sensitive(&mut self, yes: bool) {
        self.case_sensitive = yes;
    }

    pub fn exact_match(&mut self, yes: bool) {
        self.exact_match = yes;
    }

    pub fn is_exact_match(&self) -> bool {
        self.exact_match
    }

    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let case_sensitive = true;
        let exact_match = false;

        let options = Options::new(case_sensitive, exact_match);

        assert_eq!(options.case_sensitive, case_sensitive);
        assert_eq!(options.exact_match, exact_match);
    }

    #[test]
    fn test_case_sensitive() {
        let mut options = Options {
            case_sensitive: false,
            exact_match: false
        };

        options.case_sensitive(true);

        assert_eq!(options.case_sensitive, true);
        assert_eq!(options.exact_match, false);
    }

    #[test]
    fn test_exact_match() {
        let mut options = Options {
            case_sensitive: false,
            exact_match: false
        };

        options.exact_match(true);

        assert_eq!(options.exact_match, true);
        assert_eq!(options.case_sensitive, false);
    }

    #[test]
    fn test_options_is_case_sensitive() {
        let mut options = Options {
            case_sensitive: false,
            exact_match: true
        };

        assert_eq!(options.is_case_sensitive(), false);
    }

    #[test]
    fn test_options_is_exact_match() {
        let mut options = Options {
            case_sensitive: true,
            exact_match: false
        };

        assert_eq!(options.is_exact_match(), false);
    }
}
