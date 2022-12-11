use std::fs;
use std::path::Path;

// A -> Rock
// B -> Paper
// C -> Scissors
// X -> Rock
// Y -> Paper
// Z -> Scissors
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
            (_, 'X') => 1,
            (_, 'Y') => 2,
            (_, 'Z') => 3,
            _ => panic!("Invalid player move {:?}", moves),
        };

        sum += match moves {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            _ => panic!("Invalid pair of moves"),
        };

        return sum;
    });

    println!("{}", score);
}
