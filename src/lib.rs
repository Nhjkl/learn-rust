pub fn adds_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub mod mgrep {
    use std::{
        env::{self, Args},
        error::Error,
        fs,
    };

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    impl Config {
        pub fn new(mut args: Args) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("Not enough arguments");
            }
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query,
                filename,
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
        contents.lines().filter(|x| x.contains(query)).collect()
    }

    pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|x| x.to_lowercase().contains(&query))
            .collect()
    }
}
