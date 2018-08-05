use std::env;
// For handling files
use std::fs::File;
// Contains various useful traits for doing I/O, including file I/O
// In this case needed for `read_to_string`
use std::io::prelude::*;
use std::process;

// Use a struct to better associate query 
// with filename and be more explicit
struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn main() {
    // `collect` will turn an iterator into
    // a collection, in this case a Vector
    let args: Vec<String> = env::args().collect();

    // `unwrap_or_else` will unwrap the value of `Ok`, if ok,
    // otherwise it will call the callback, accepting `err`
    // and exit the process
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}…\n", config.query, config.filename);

    // `expect` is needed/important, otherwise `read_to_string` will fail
    let mut f = File::open(config.filename).expect("File not found.");
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Something went wrong reading the file.");

    println!("{}", content);
}
