use learning_rust::mgrep::{self, Config};

#[test]
fn it_parse_config() {
    let args = vec!["".to_string(), "query".to_string(), "filename".to_string()];

    let config = Config::new(&args).unwrap_or_else(|err| {
        panic!("Error: {}", err);
    });

    assert_eq!(config.query, "query");
    assert_eq!(config.filename, "filename");
}

#[test]
#[should_panic(expected = "Not enough arguments")]
fn it_parse_config_panic() {
    let args = vec!["".to_string()];
    Config::new(&args).unwrap_or_else(|err| {
        panic!("Error: {}", err);
    });
}

#[test]
fn it_one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        mgrep::search(query, contents)
    );
}

#[test]
fn it_case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        mgrep::search(query, contents)
    );
}

#[test]
fn it_case_insensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive.", "Duct tape."],
        mgrep::search_case_insensitive(query, contents)
    );
}
