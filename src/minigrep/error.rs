use std::error::Error;
use std::fmt;

/// Minigrep error.
#[derive(Debug)]
pub struct MinigrepError {
    message: String,
}

impl MinigrepError {
    pub fn new(message: &str) -> Self {
        MinigrepError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for MinigrepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MinigrepError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minigrep_error_new() {
        let minigrep_error = MinigrepError::new("the reason it failed");

        assert_eq!(minigrep_error.message, "the reason it failed");
    }
}
