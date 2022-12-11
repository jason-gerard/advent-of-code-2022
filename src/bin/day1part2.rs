use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day1-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let elf_sums: Vec<i32> = contents
        .trim()
        .split("\n\n")
        .map(|calorie_lines| calorie_lines
            .split("\n")
            .map(|calorie_line| calorie_line.parse::<i32>().unwrap())
            .sum())
        .collect();

    let mut elf_sums_sorted = elf_sums.clone();
    elf_sums_sorted.sort();

    println!("{:?}", elf_sums_sorted);
    let most_calories: i32 = elf_sums_sorted.iter().rev().take(3).sum();

    println!("{}", most_calories);
}
