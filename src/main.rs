use std::env;
// For handling files
use std::fs::File;
// Contains various useful traits for doing I/O, including file I/O
use std::io::prelude::*;

// Use a struct to better associate query 
// with filename and be more explicit
struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config {
            query,
            filename
        }
    }
}

fn main() {
    // `collect` will turn an iterator into
    // a collection, in this case a Vector
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {} in {}", config.query, config.filename);

    // `expect` is needed/important, otherwise `read_to_string` will fail
    let mut f = File::open(config.filename).expect("File not found.");
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Something went wrong reading the file.");

    println!("{}", content);
}
