use learning_rust::mgrep::{self};

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
        mgrep::search(query, contents)
    );
}
