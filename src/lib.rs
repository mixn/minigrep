#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}

// For handling files
use std::fs::File;
// Contains various useful traits for doing I/O, including file I/O
// In this case needed for `read_to_string`
use std::io::prelude::*;
use std::error::Error;
use std::env;

// Use a struct to better associate query 
// with filename and be more explicit
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn’t get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn’t get a file name")
        };

        // Returns a Result that will be the successful `Ok` variant
        // that contains the value of the environment variable if the environment variable is set
        // Returns `Err` variant if the environment variable is not set
        // `is_err` is used to whether it’s an error and therefore unset,
        // we only care about set or unset, hence `is_err` over `unwrap` or `expect`
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // “Resolve” with an instance of `Config`
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // `?` instead of `expect`, as we don’t wanna panic right away
    let mut f = File::open(config.filename)?;
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Something went wrong reading the file.");

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Needed lifetime 'a since lifetime of `content` is connected to
// the lifetime of the return value. We indicate that the returned
// vector should contain string slices that reference slices of
// the argument `content` (rather than the argument `query`)
// We tell Rust that the data returned by `search` will live as long
// as the data passed into the `search` function in the `content` argument
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
