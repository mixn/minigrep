extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // `unwrap_or_else` will unwrap the value of `Ok`, if ok,
    // otherwise it will call the callback, accepting `err`
    // and exit the process
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}…\n", config.query, config.filename);

    // Error-handling code, since `run` returns a `Result`
    // We only care about the failing case, as success is just `()`
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
