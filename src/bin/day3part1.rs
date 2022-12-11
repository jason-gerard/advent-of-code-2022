use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day3-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let priorities_by_ruck = contents
        .trim()
        .split("\n")
        .map(|line| line.chars().map(letter_to_priority).collect());

    let summed_duplicated_priorities: u32 = priorities_by_ruck
        .map(|priorities: Vec<u32>| {
            let (first, second) = priorities.split_at(priorities.len() / 2);

            let x = HashSet::<u32>::from_iter(first.iter().cloned());
            let y = HashSet::<u32>::from_iter(second.iter().cloned());
            return x.intersection(&y).cloned().collect::<Vec<u32>>()[0];
        }).sum();

    println!("{}", summed_duplicated_priorities);
}

fn letter_to_priority(letter: char) -> u32 {
    let ascii = letter as u32;
    if ascii >= 97 { ascii - 96 } else { ascii - 38 }
}
