use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day3-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let lines = contents
        .trim()
        .split("\n")
        .map(|line| line.chars().map(letter_to_priority).collect())
        .collect::<Vec<Vec<u32>>>();


    let mut group_badges = Vec::<u32>::new();

    let groups = lines.chunks(3);
    for group in groups {
        let mut group_intersections = group.first().unwrap().clone();

        for ruck in group {
            let unique_ruck: HashSet<u32> = ruck.clone().into_iter().collect();
            group_intersections = unique_ruck
                .intersection(&group_intersections.into_iter().collect::<HashSet<u32>>())
                .map(|i| *i)
                .collect::<Vec<_>>();
        }

        group_badges.push(*group_intersections.first().unwrap());
    }

    println!("Sum of priorities: {:?}", group_badges.iter().sum::<u32>());
}

fn letter_to_priority(letter: char) -> u32 {
    let ascii = letter as u32;
    if ascii >= 97 { ascii - 96 } else { ascii - 38 }
}
