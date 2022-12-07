use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day6-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    println!("{}", contents);
    
    let window_size = 4;
    
    let buffer = contents.trim();
    for i in 0..buffer.len() {
        let substring = buffer.chars().skip(i).take(window_size).collect::<Vec<char>>();
        let unique_substring = HashSet::<char>::from_iter(substring.iter().cloned());
        
        if substring.len() == unique_substring.len() {
            println!("{} is the start of the data stream", i + window_size);
            break
        }
    }
}
