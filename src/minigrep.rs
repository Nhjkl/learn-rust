use learning_rust::mgrep::{self, Config};
use std::env;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        panic!("Error: {}", err);
    });

    if let Err(e) = mgrep::run(config) {
        panic!("{}", e);
    }
}
