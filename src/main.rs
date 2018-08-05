use std::env;
// For handling files
use std::fs::File;
// Contains various useful traits for doing I/O, including file I/O
use std::io::prelude::*;

fn main() {
    // `collect` will turn an iterator into
    // a collection, in this case a Vector
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    let filename = &args[2];
    // `expect` is needed/important, otherwise `read_to_string` will fail
    let mut f = File::open(filename).expect("File not found.");
    let mut content = String::new();

    f.read_to_string(&mut content).expect("Something went wrong reading the file.");

    println!("{}", content);
}
