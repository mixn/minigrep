// For handling files
use std::fs::File;
// Contains various useful traits for doing I/O, including file I/O
// In this case needed for `read_to_string`
use std::io::prelude::*;
use std::error::Error;

// Use a struct to better associate query 
// with filename and be more explicit
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // Abort if not at least 2 arguments given,
        // first argument is always `target/debug/binary-name`
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // “Resolve” with an instance of `Config`
        Ok(Config { query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // `?` instead of `expect`, as we don’t wanna panic right away
    let mut f = File::open(config.filename)?;
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Something went wrong reading the file.");

    println!("{}", content);

    Ok(())
}