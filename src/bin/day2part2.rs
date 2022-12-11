use std::fs;
use std::path::Path;

// A -> Rock
// B -> Paper
// C -> Scissors
// X -> Lose
// Y -> Draw
// Z -> Win
fn main() {
    let path = Path::new("./src/bin/inputs/day2-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let rounds = contents
        .trim()
        .split("\n")
        .map(|round| (round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap()));
    let score = rounds.fold(0, |mut sum, moves| {
        sum += match moves {
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 0 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            _ => panic!("Invalid player move {:?}", moves),
        };

        return sum;
    });

    println!("{}", score);
}
