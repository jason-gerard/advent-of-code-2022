use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day4-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let count = contents
        .trim()
        .split("\n")
        .map(|line| {
            // [[start, end],[start, end]]
            let ranges = line
                .split(",")
                .flat_map(|part| part.split("-"))
                .map(|val| val.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            // if first end < second start or second end < first start
            // they dont overlap
            if !(ranges[1] < ranges[2] || ranges[3] < ranges[0]) { 1 } else { 0 }
        })
        .sum::<u32>();

    println!("{} number of ranges overlap", count);
}
