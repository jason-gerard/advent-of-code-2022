use std::fs;
use std::path::Path;

fn main() {
    println!("Hello, world! Day1");
    let path = Path::new("./src/bin/day1-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    println!("{}", contents);
}
