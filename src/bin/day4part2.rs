use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/day4-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let count: u32 = contents
        .trim()
        .split("\n")
        .map(|line| {
            // [[start, end],[start, end]]
            let mut x = line.split(",").map(|part| part.split("-"));

            let mut first = x.next().unwrap();
            let first_start = first.next().unwrap().parse::<u32>().unwrap();
            let first_end = first.next().unwrap().parse::<u32>().unwrap();

            let mut second = x.next().unwrap();
            let second_start = second.next().unwrap().parse::<u32>().unwrap();
            let second_end = second.next().unwrap().parse::<u32>().unwrap();

            // if first end < second start or second end < first start
            // they dont overlap
            if !(first_end < second_start || second_end < first_start) { 1 } else { 0 }
        })
        .sum();

    println!("{} number of ranges", count);
}
