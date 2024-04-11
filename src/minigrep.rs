use learning_rust::mgrep::{self, Config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        panic!("Error: {}", err);
    });

    if let Err(e) = mgrep::run(config) {
        panic!("{}", e);
    }
}
