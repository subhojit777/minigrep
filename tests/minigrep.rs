extern crate minigrep;

use minigrep::*;
use std::io::Read;

#[test]
fn test_bin_too_few_args() {
    let args = [String::from("command"), String::from("too few arguments")];
    let config = parse_config(&args);

    assert!(config.is_err());
}

#[test]
fn test_bin_incorrect_options() {
    let args = [
        String::from("command"),
        String::from("-incorrect"),
        String::from("query"),
        String::from("./test-data/test.txt"),
    ];
    let config = parse_config(&args);

    assert!(config.is_err());
}

#[test]
fn test_bin_incorrect_file() {
    let args = [
        String::from("command"),
        String::from("-w"),
        String::from("query"),
        String::from("../test-data/does-not-exist.txt"),
    ];
    let config = parse_config(&args);

    assert!(config.is_err());
}

#[test]
fn test_bin() {
    let args = [
        String::from("command"),
        String::from("-w"),
        String::from("is"),
        String::from("./test-data/test.txt"),
    ];
    let config = parse_config(&args).unwrap();
    let mut file_content = String::new();
    config.get_file().read_to_string(&mut file_content).unwrap();
    let matched_indices = search(&config);

    assert_eq!(matched_indices.len(), 1);
    assert_eq!(matched_indices[0], 5);
}
