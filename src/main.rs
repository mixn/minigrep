use std::env;

fn main() {
    // `collect` will turn an iterator into
    // a collection, in this case a Vector
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
