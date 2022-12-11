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

    let most_calories = elf_sums.iter().max().unwrap();
    let index = elf_sums.iter().position(|element| element == most_calories).unwrap();

    println!("{} at index {}", most_calories, index);
}
