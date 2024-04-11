pub fn adds_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub mod mgrep {
    use std::{env, error::Error, fs};

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &str> {
            if args.len() < 3 {
                return Err("Not enough arguments");
            }
            let query = &args[1];
            let filename = &args[2];

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query: query.to_string(),
                filename: filename.to_string(),
                case_sensitive,
            })
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("match: {}", line);
        }

        Ok(())
    }

    pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut matchs = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                matchs.push(line);
            }
        }

        matchs
    }

    pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut matchs = Vec::new();

        let query = query.to_lowercase();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                matchs.push(line);
            }
        }

        matchs
    }
}
